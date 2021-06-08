use spin::RwLock;



pub mod thread;
pub mod proc;


pub static SCHEDULE: RwLock<bool> = RwLock::new(false);