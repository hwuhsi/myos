
use x86_64::{VirtAddr, registers::control::Cr3, structures::paging::{OffsetPageTable, PageTable}};
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) 
    -> &'static mut PageTable
{
    
    let (frame, _) = Cr3::read();
    let phys = frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    
    &mut *page_table_ptr
}


pub unsafe fn init_page_table(physical_memory_offset: VirtAddr)
    -> OffsetPageTable<'static>
{
    
    let table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(table, physical_memory_offset)
}


