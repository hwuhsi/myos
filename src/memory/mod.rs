
pub mod addr;
pub mod bitalloc;

use bitalloc::BitAlloc1M;
use lazy_static::lazy_static;
use linked_list_allocator::LockedHeap;

use spin::Mutex;

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

lazy_static!(
    pub static ref BITMAP_ALLOCATOR: Mutex<BitAlloc1M> = Mutex::new(BitAlloc1M::default());
);