use bootloader::{BootInfo, entry_point};
use cpu::halt;
use interrupt::int::init_idt;
use x86_64::structures::paging::{OffsetPageTable, PageTable};
use crate::{memory::{BITMAP_ALLOCATOR, addr::phys_to_virt, bitalloc::BitAlloc}, process::proc::do_print_hello};
use crate::process::proc::{create_kernel_process, create_kernel_process2, init_kernel_process};

use self::{interrupt::ctx::Context, memory::mem_init, pci::init_pci};
use crate::process::SCHEDULE;

pub mod partition;
pub mod consts;
pub mod memory;
pub mod vga;
pub mod cpu;
pub mod interrupt;
pub mod page;
pub mod pci;
pub mod ahci;


entry_point!(kernel_main);

fn kernel_main(bootinfo: &'static BootInfo) -> ! {
    
    mem_init(bootinfo);
    init_kernel_process();
    init_idt();
    init_pci();
    create_kernel_process(1);
    let addr = BITMAP_ALLOCATOR.lock().alloc().unwrap();
    let addr2 = BITMAP_ALLOCATOR.lock().alloc().unwrap();
    let addrv = phys_to_virt(addr2);
    let ctx = Context {
        cr2: 0,
        r15: 0,
        r14: 0,
        r13: 0,
        r12: 0,
        r11: 0,
        r10: 0,
        r9: 0,
        r8: 0,
        rbp: 0,
        rdi: 0,
        rsi: 0,
        rdx: 0,
        rcx: 0,
        rbx: 0,
        rax: 0,
        rsp: addrv as u64,
        ip: do_print_hello as u64,
    };
    
    //let proc = PROCESSES.write();
    create_kernel_process2(1, ctx);
    println!("{:?}", ctx);
    {
        let mut x = SCHEDULE.write();
        *x = true;
    }
    
    loop {
        halt();
    }
}