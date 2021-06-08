

use bootloader::{BootInfo, bootinfo::MemoryRegionType};
use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB}};

use crate::memory::bitalloc::{BitAlloc, BitAlloc1M};

use super::consts::{KERNEL_HEAP_SIZE, KERNEL_HEAP_START};

use super::page::init_page_table;


use crate::memory::HEAP_ALLOCATOR;
use crate::memory::BITMAP_ALLOCATOR;

/// init frame allocator and heap 
pub fn mem_init(bootinfo: &'static BootInfo) {
    bitalloc_init(bootinfo);
    let mut table = unsafe { init_page_table(VirtAddr::new(0xFFFF800000000000)) };
    let page: Page<Size4KiB> = Page::from_start_address(VirtAddr::new(0xfee00000)).unwrap();
    let frame: PhysFrame<Size4KiB> = PhysFrame::from_start_address(PhysAddr::new(0xfee00000)).unwrap();
    unsafe {
        let mut b = BITMAP_ALLOCATOR.lock();
        table.map_to(page, frame, PageTableFlags::NO_CACHE | PageTableFlags::WRITABLE | PageTableFlags::PRESENT, &mut *b).expect("msg").flush();
    }


    unsafe {
        let mut b = BITMAP_ALLOCATOR.lock();
        let phyaddr  = b.alloc_contiguous(KERNEL_HEAP_SIZE / 0x1000, 10).unwrap() * 0x1000;
        println!("heap physics address: {:#x}", phyaddr);
        for i in 0..KERNEL_HEAP_SIZE / 0x1000 {
            let addr: u64 = KERNEL_HEAP_START as u64 + i as u64 * 0x1000;
            let page: Page<Size4KiB> = Page::from_start_address(
                VirtAddr::new(addr)
            ).unwrap();
            let phys: u64 = phyaddr as u64 + i as u64 * 0x1000;
            let frame: PhysFrame<Size4KiB> = PhysFrame::from_start_address(
                PhysAddr::new(phys)
            ).unwrap();
            table.map_to(page, frame, PageTableFlags::PRESENT | PageTableFlags::WRITABLE, &mut *b).unwrap().flush();
        }
    }
    unsafe { HEAP_ALLOCATOR.lock().init(KERNEL_HEAP_START, KERNEL_HEAP_SIZE) };

    println!("successfully init heap\nheap start addr: {:#x}\nheap size: {:#x}", KERNEL_HEAP_START, KERNEL_HEAP_SIZE);

}


pub fn bitalloc_init(bootinfo: &'static BootInfo) {
    let j =  bootinfo.memory_map.iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable)
        .map(|r| r.range.start_addr() as usize / 0x1000..r.range.end_addr() as usize / 0x1000);
         
    {
        let mut block = BITMAP_ALLOCATOR.lock();
        for i in j {
            println!("bit allocator find block {:#x}~{:#x}", i.start, i.end);
            block.insert(i);
        }

    }
}


unsafe impl FrameAllocator<Size4KiB> for BitAlloc1M {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        if let Some(s) = self.alloc() {
            PhysFrame::from_start_address(PhysAddr::new(s as u64 * 0x1000)).ok()
        }else {
            None
        }

    }
}