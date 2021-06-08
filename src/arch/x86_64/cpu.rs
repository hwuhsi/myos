use x86_64::registers::control::Cr2;

pub fn halt() {
    unsafe {
        asm!(
            "hlt"
        );
    }
}

pub fn get_page_fault_addr() -> usize {
    Cr2::read().as_u64() as usize
}


/// disable pic to use more advanced apic
pub fn disable_pic() {
    unsafe {
        asm!(
            "mov al, 0xff",
            "out 0xa1, al",
            "out 0x21, al"
        )
    }
}