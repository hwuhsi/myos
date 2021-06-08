/* 

use alloc::sync::Arc;
use bitvec::prelude::*;
use rcore_fs::dev::{BlockDevice, Device};

#[repr(C, packed)]
struct MBRStruct {
    boot: [u8; 446],
    pub entrys: [PartitionEntry; 4],
    pub signature: [u8; 2]
}

#[repr(C, packed)]
struct PartitionEntry {
    status: u8,
    first_chs: [u8; 3],
    partitiontype: u8,
    last_chs: [u8; 3],
    pub first_lba: u32,
    pub size: u32
}

pub unsafe fn do_partition(dev: Arc<dyn Device>) {
    let mut ve: [u8; 512] = [0; 512];

    let p = dev.read_at(0, ve.as_mut_slice()).unwrap();
    let t: MBRStruct = core::mem::transmute_copy(ve.as_ptr().as_ref().unwrap());
    println!("{:?}", ve);
    println!("first lba is {}", t.entrys[0].first_lba);
    println!("the size is {}", t.entrys[0].size);
}
*/