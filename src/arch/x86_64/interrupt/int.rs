
use core::{intrinsics::volatile_copy_nonoverlapping_memory, mem::size_of};

use crate::{arch::cpu::disable_pic, process::SCHEDULE};

use apic::LocalApic;
use lazy_static::lazy_static;

use crate::process::proc::PROCESSES;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use super::{ctx::Context, trap::TrapFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[32].set_handler_fn(unsafe {core::mem::transmute(irq0 as extern "C" fn())});
        
        idt
    };
    //static ref LAPIC: apic::XApic = unsafe { apic::XApic::new(0xeee) };
}


pub fn init_idt() {

    IDT.load();
    disable_pic();
    x86_64::instructions::interrupts::enable();

    let mut me = unsafe { apic::XApic::new(0xfee00000) };
    me.cpu_init();
    //println!("hello world");
}




extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}



extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(_stack_frame: &mut InterruptStackFrame, _error_code: PageFaultErrorCode) {
    println!("{:?}", _error_code);
    panic!("page fault");
    
}



#[naked]
extern "C" fn irq0() {
    unsafe {
    asm!(
        "push rsp",
        "push rax",
        "mov rax, 0",
        "call irq_common",
        options(noreturn)
    )
    }
}

/// adopted from https://gist.github.com/mark-i-m/361cbcc39769f965b1c419091b9cbf4f#file-machine-rs-L248
#[no_mangle]
#[naked]
unsafe extern "C" fn irq_common() {
    asm!(
        "add rsp, 8",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push rbp",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "mov rbp, cr2",
        "push rbp",
        "mov rdi, rax",
        "mov rsi, rsp",
        "call myfun",
        "pop rbp",
        "mov cr2, rbp",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        "add rsp, 8",
        "iretq",
        options(noreturn)
    );
}


/// Handle interrupt
#[no_mangle]
extern "sysv64" fn myfun(_irq: u64, context_ptr: *mut TrapFrame) {
    
    //print!(".");
    //println!("irq is {:#x}", irq);
    //unsafe { println!("{:?}", *context_ptr) };
    let mut me = unsafe { apic::XApic::new(0xfee00000) };
    match SCHEDULE.try_read() {
        Some(d) => {
            if *d {
                //println!("lf7j83b56tn37gr2n76ty");
                let ps = PROCESSES.read();
                //println!("lf7j83b56tn37gr2n76ty");
                let p = ps.get(&1).unwrap();
                let ctx = p.ctx;
                let ptr = & ctx as *const Context;
                //unsafe { println!("first {:?}, second {:?}", ptr, context_ptr) };
                
                unsafe { volatile_copy_nonoverlapping_memory(context_ptr, ptr, 1); };
                
                
            }
            
        }

        None => {}
    }
    
    me.eoi();
    
    
}

#[inline]
pub fn disable_and_store() -> u64 {
    let u: u64;
    //println!("he");
    unsafe {
        asm!(
            "pushfq",
            "pop {}",
            "cli",
            lateout(reg) u
        )
    }
    u
}

#[inline]
pub fn restore(eflags: u64) {
    //println!("wd");
    unsafe {
        asm!(
            "push {0}",
            "popfq",
            in(reg) eflags
        )
    }
}