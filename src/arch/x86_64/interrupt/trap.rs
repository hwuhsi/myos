

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TrapFrame {
    pub(crate) cr2: u64,
    pub(crate) r15: u64,
    pub(crate) r14: u64,
    pub(crate) r13: u64,
    pub(crate) r12: u64,
    pub(crate) r11: u64,
    pub(crate) r10: u64,
    pub(crate) r9: u64,
    pub(crate) r8: u64,
    pub(crate) rbp: u64,
    pub(crate) rdi: u64,
    pub(crate) rsi: u64,
    pub(crate) rdx: u64,
    pub(crate) rcx: u64,
    pub(crate) rbx: u64,
    pub(crate) rax: u64,
    pub(crate) rsp: u64,
    pub(crate) ip: u64
}