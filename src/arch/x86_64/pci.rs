use crate::drivers::pci::{BAR, ConfigSpaceAccessMethod::IO, PCIDevice, PortOps, scan_bus};
use x86_64::instructions::port::Port;


struct PortOpsImpl;


impl PortOps for PortOpsImpl {


    unsafe fn read32(&self, port: u16) -> u32 {
        Port::new(port).read()
    }


    unsafe fn write32(&self, port: u16, val: u32) {
        Port::new(port).write(val)
    }
}


pub fn init_pci() {
    println!("begin scan");
    
    let pci = unsafe { scan_bus(&PortOpsImpl, IO) };
    for dev in pci {
        println!(
            "pci: {:02x}:{:02x}.{} {:#x} {:#x} ({} {}) irq: {}:{:?}",
            dev.loc.bus,
            dev.loc.device,
            dev.loc.function,
            dev.id.vendor_id,
            dev.id.device_id,
            dev.id.class,
            dev.id.subclass,
            dev.pic_interrupt_line,
            dev.interrupt_pin,
        );
        init_driver(&dev);
        
    }
}

pub fn init_driver(dev: &PCIDevice) {
    if dev.id.class == 0x1 && dev.id.subclass == 0x6 {
        
        if let Some(BAR::Memory(addr, len, d, c)) = dev.bars[5] {
            println!("Found AHCI dev {:?} BAR5 {:x?}", dev, addr);
        }
    }
}


