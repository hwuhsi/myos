
use lazy_static::lazy_static;
use spin::RwLock;
use alloc::vec::Vec;
use alloc::sync::Arc;
use alloc::string::String;

use self::block::BlockDriver;

pub mod block;
pub mod pci;
pub trait SomeTrait: Send + Sync {
    fn some(&self);
}

lazy_static!(
    pub static ref BLK_DRIVERS: RwLock<Vec<Arc<dyn BlockDriver>>> = RwLock::new(Vec::new());
);

#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Net,
    Gpu,
    Input,
    Block,
    Rtc,
    Serial,
    Intc,
}

pub trait Driver: Send + Sync {
    // if interrupt belongs to this driver, handle it and return true
    // return false otherwise
    // irq number is provided when available
    // driver should skip handling when irq number is mismatched
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool;

    // return the correspondent device type, see DeviceType
    fn device_type(&self) -> DeviceType;

    // get unique identifier for this device
    // should be different for each instance
    fn get_id(&self) -> String;

    // trait casting
    fn as_block(&self) -> Option<&dyn BlockDriver> {
        None
    }
}