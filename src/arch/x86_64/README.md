## arch x86_64 Module


### const

**arch specific consts**

PHYSICAL_MEMORY_OFFSET = 0xFFFF800000000000

ARCH = x86_64

PAGE_SIZE = 4kb

KERNEL_HEAP_START = 0x44444444_0000
KERNEL_HEAP_SIZE = 1Mb

### memory

arch specific memory related function

- [x] bitmap allocator
- [ ] buddy system allocator
- [ ] slab allocator

### interrupt

support apic by [apic](https://github.com/rcore-os/apic-rs) repo

todo

- [ ] context switch 



### pci

thanks to [pci-rs](https://github.com/rcore-os/pci-rs)

- [x] probe
- [ ] set up

### vga

print to vga console