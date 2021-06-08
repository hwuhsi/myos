use alloc::sync::Arc;
use isomorphic_drivers::{block::ahci::AHCI, provider::Provider};
//use rcore_fs::dev::{BlockDevice, BlockId, DevError};
use spin::Mutex;
use alloc::string::String;
use crate::{drivers::{Driver, block::BlockDriver}, memory::{BITMAP_ALLOCATOR, addr::{phys_to_virt, virt_to_phys}, bitalloc::BitAlloc}};

struct MyProvider;

impl Provider for MyProvider {
    const PAGE_SIZE: usize = 0x1000;

    fn alloc_dma(size: usize) -> (usize, usize) {
        //println!("alloc dma begin!!! {}", size);
        let s  = BITMAP_ALLOCATOR.lock().alloc_contiguous(size / 0x1000, 8).unwrap() * 0x1000;
        //println!("alloc end !!!{:#x}", s);
        (phys_to_virt(s), s)
    }

    fn dealloc_dma(vaddr: usize, size: usize) {
        //println!("dealloc begin, {:#x}, {}", vaddr, size);
        let jj = size / 0x1000;
        for j in 0..jj {
            
            let addr = virt_to_phys(vaddr) + j * 0x1000;
            //println!("www {:#x}", addr);
            BITMAP_ALLOCATOR.lock().dealloc(addr / 0x1000);
        }
    }
}

pub struct AHCIDriver(Mutex<AHCI<MyProvider>>);


impl AHCIDriver {
    pub fn write(&self,block_id: usize, buf: &[u8]) {
        let mut loc = self.0.lock();
        loc.write_block(block_id, buf);
    }

    pub fn read(&self, block_id: usize, buf: &mut [u8]) {
        let mut loc = self.0.lock();
        loc.read_block(block_id, buf);
    }
}

impl Driver for AHCIDriver {
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool {
        todo!()
    }

    fn device_type(&self) -> crate::drivers::DeviceType {
        todo!()
    }

    fn get_id(&self) -> String {
        String::from("ahci")
    }

    fn as_block(&self) -> Option<&dyn BlockDriver> {
        Some(self)
    }
}

impl BlockDriver for AHCIDriver {
    fn read_at(&self, block_id: usize, buf: &mut [u8]) {
        self.read(block_id, buf);
    }
    fn write_at(&self, block_id: usize, buf: &[u8]) {
        self.write(block_id, buf);
    }
}

pub fn init(_irq: Option<usize>, header: usize, size: usize) -> Option<Arc<AHCIDriver>> {
    if let Some(ahci) = AHCI::new(header, size) {
        let driver = Arc::new(AHCIDriver(Mutex::new(ahci)));
        Some(driver)
    } else {
        None
    }
}