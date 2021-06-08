use crate::arch::consts::PHYSICAL_MEMORY_OFFSET;

#[inline]
pub const fn phys_to_virt(phys: usize) -> usize {
    phys + PHYSICAL_MEMORY_OFFSET
}

#[inline]
pub const fn virt_to_phys(virt: usize) -> usize {
    virt - PHYSICAL_MEMORY_OFFSET
}