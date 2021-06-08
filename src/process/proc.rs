use alloc::{collections::BTreeMap, vec::Vec};
use alloc::sync::Arc;
use lazy_static::lazy_static;
use spin::RwLock;

use crate::{arch::interrupt::ctx::Context, consts::{MAX_CPU_NUM, MAX_PROCESS_NUM}};


pub struct Process {
    /// arch specific context
    pub ctx: Context,
    pid: usize,
    state: ProcessState,
    is_kernel: bool,
}

impl Process {
    pub fn set_ctx(&mut self, ctx: Context) {
        self.ctx = ctx;
    }
}

#[derive(Debug)]
pub enum ProcessState {
    Busy,
    Wait,
    Running
}

lazy_static!{
    pub static ref PROCESSES: RwLock<BTreeMap<usize, Arc<Process>>> = RwLock::new(BTreeMap::new());
}

#[inline(always)]
pub fn init_kernel_process() {
    create_kernel_process(0);
}

pub fn create_kernel_process(pid: usize) {
    let proc = Process {
        ctx: Context::default(),
        pid,
        state: ProcessState::Running,
        is_kernel: true
    };
    PROCESSES.write().insert(pid, Arc::new(proc));
}

pub fn create_kernel_process2(pid: usize, ctx: Context) {
    let proc = Process {
        ctx,
        pid,
        state: ProcessState::Running,
        is_kernel: true
    };
    PROCESSES.write().insert(pid, Arc::new(proc));
}
 
pub fn do_print_hello() {
    println!("hello world from context switch!");
}


pub fn switch_to_process1() {
    let d = PROCESSES.read();
    let p = d.get(&1).unwrap();
    
}
// process list used for task scheduling
//pub static PROCESS_LISTS: [Option<Arc<Process>>; MAX_CPU_NUM] = [None; MAX_CPU_NUM];



