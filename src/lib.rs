#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(array_methods)]
#![feature(core_intrinsics)]
extern crate alloc;


#[macro_use]
extern crate bitflags;

#[macro_use]
pub mod lang;
pub mod consts;
pub mod memory;
pub mod fs;
pub mod process;

#[path = "arch/x86_64/mod.rs"]
pub mod arch;
pub mod sync;
pub mod drivers;

use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
