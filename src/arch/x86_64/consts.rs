
pub const PHYSICAL_MEMORY_OFFSET: usize = 0xFFFF800000000000;

pub const ARCH: &'static str = "x86_64";

pub const PAGE_SIZE: usize = 0x1000;

pub const KERNEL_HEAP_START: usize = 0x_4444_4444_0000;
pub const KERNEL_HEAP_SIZE: usize = 1 * 1024 * 1024; // 1 MB