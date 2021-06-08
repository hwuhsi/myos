

use core::{cell::UnsafeCell, fmt, hint, mem::MaybeUninit, ops::{Deref, DerefMut}, sync::atomic::{AtomicBool, AtomicU8, Ordering}, u64};


use interrupt::int::{disable_and_store, restore};
use crate::arch::interrupt;

pub type SpinNoIrqLock<T> = Mutex<T, SpinNoIrq>;
pub type SpinLock<T> = Mutex<T, Spin>;  


pub struct Mutex<T: ?Sized, S: MutexSupport> {
    lock: AtomicBool,
    support: MaybeUninit<S>,
    support_initialization: AtomicU8, // 0 = uninitialized, 1 = initializing, 2 = initialized
    user: UnsafeCell<(usize, usize)>, // (cid, tid)
    data: UnsafeCell<T>,
}

#[allow(dead_code)]
pub struct MutexGuard<'a, T: ?Sized + 'a, S: MutexSupport + 'a> {
    pub(super) mutex: &'a Mutex<T, S>,
    support_guard: S::GuardData,
}

/// Low-level support for mutex
pub trait MutexSupport {
    type GuardData;
    fn new() -> Self;
    /// Called when failing to acquire the lock
    fn cpu_relax(&self);
    /// Called before lock() & try_lock()
    fn before_lock() -> Self::GuardData;
    /// Called when MutexGuard dropping
    fn after_unlock(&self);
}

unsafe impl <T, S>Sync for Mutex<T, S> where 
    T: ?Sized + Send,
    S: MutexSupport
{}

unsafe impl <T, S>Send for Mutex<T, S> where 
    T: ?Sized + Send,
    S: MutexSupport
{}

impl<T, S: MutexSupport> Mutex<T, S> {
    pub const fn new(data: T) -> Mutex<T, S>{
        Mutex {
            lock: AtomicBool::new(false),
            support: MaybeUninit::uninit(),
            support_initialization: AtomicU8::new(0),
            user: UnsafeCell::new((0, 0)),
            data: UnsafeCell::new(data),
            
        }
    }

    /// Consumes this mutex, returning the underlying data.
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let Mutex { data, .. } = self;
        data.into_inner()
    }
}

impl<T: ?Sized, S: MutexSupport> Mutex<T, S> {
    fn obtain_lock(&self) {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            let mut try_count = 0;
            while self.lock.load(Ordering::Relaxed) {
                unsafe { &*self.support.as_ptr() }.cpu_relax();
                try_count += 1;
                if try_count == 0x100000 {
                    let (_cid, _tid) = unsafe { *self.user.get() };
                    crate::println!("error detected!");
                }
            }
        }
        
    }

    pub fn ensure_support(&self) {
        let initialization = self.support_initialization.load(Ordering::Relaxed);
        if initialization == 2 {
            return;
        };
        if initialization == 1
            || self
                .support_initialization
                .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).expect("msg")
                != 0
        {
            // Wait for another thread to initialize
            while self.support_initialization.load(Ordering::Acquire) == 1 {
                hint::spin_loop();
            }
        } else {
            // My turn to initialize
            (unsafe { core::ptr::write(self.support.as_ptr() as *mut _, S::new()) });
            self.support_initialization.store(2, Ordering::Release);
        }
    }


    pub fn lock(&self) -> MutexGuard<T, S>{
        let support_guard = S::before_lock();
        self.ensure_support();
        self.obtain_lock();
        MutexGuard {
            mutex: self,
            support_guard,
        }
    }
    pub fn busy_lock(&self) -> MutexGuard<T, S> {
        loop {
            if let Some(x) = self.try_lock() {
                break x
            }
        }
    }
    pub fn try_lock(&self) -> Option<MutexGuard<T, S>> {
        let support_guard = S::before_lock();
        if self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            Some(MutexGuard {
                mutex: self,
                support_guard,
            })
        } else {
            None
        }
    }
}


impl<'a, T: ?Sized, S: MutexSupport> DerefMut for MutexGuard<'a, T, S> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized, S: MutexSupport> Deref for MutexGuard<'a, T, S> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}


impl<'a, T: ?Sized, S: MutexSupport> Drop for MutexGuard<'a, T, S> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
        unsafe { &*self.mutex.support.as_ptr() }.after_unlock();
    }
}

#[derive(Debug)]
pub struct Spin;

impl MutexSupport for Spin {
    type GuardData = ();

    fn new() -> Self {
        Spin
    }
    fn cpu_relax(&self) {
        hint::spin_loop()
    }
    fn before_lock() -> Self::GuardData {}
    fn after_unlock(&self) {}
}


#[derive(Debug)]
pub struct SpinNoIrq;

pub struct FlagsGuard(u64);



impl FlagsGuard {
    pub fn no_irq_region() -> Self {
        Self(disable_and_store())
    }
}


impl Drop for FlagsGuard {
    fn drop(&mut self) {
        restore(0);
    }
}

impl MutexSupport for SpinNoIrq {
    type GuardData = FlagsGuard;
    fn new() -> Self {
        SpinNoIrq
    }
    fn cpu_relax(&self) {
        hint::spin_loop();
    }
    fn before_lock() -> Self::GuardData {
        FlagsGuard(disable_and_store())
    }
    fn after_unlock(&self) {}
}


impl<T: ?Sized + fmt::Debug, S: MutexSupport + fmt::Debug> fmt::Debug for Mutex<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //self.try_lock()
        match self.try_lock() {
            Some(guard) => write!(
                f,
                "Mutex {{ data: {:?}, support: {:?} }}",
                &*guard, self.support
            ),
            None => write!(f, "Mutex {{ <locked>, support: {:?} }}", self.support),
        }
    }
}