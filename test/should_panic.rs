#![no_std]
#![no_main]



#[no_mangle]
pub extern "C" fn _start() {
    should_fail()
}

fn should_fail() {
    
}