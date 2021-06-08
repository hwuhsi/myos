/*
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use vfs::FileSystem;
use core::ops::Range;
use ext2::{error::Error, fs::sync::Inode};
use ext2::fs::sync::Synced;
use ext2::fs::Ext2;
use ext2::sector::{Address, Size512};
use ext2::volume::size::Size;
use ext2::volume::{Volume, VolumeCommit, VolumeSlice};
use rcore_fs::dev::{DevError, Device};
use rcore_fs::vfs;


#[derive(Clone)]
struct Ext2Volume {
    inner: Arc<dyn Device>,
}

#[derive(Clone)]
pub struct Ext2FileSystem {
    inner: Synced<Ext2<Size512, Ext2Volume>>,
    volume: Ext2Volume,
}

/// A conversion between vfs::FsError and ext2::Error
#[derive(Debug)]
struct Ext2Error {
    inner: Error,
}

impl core::convert::From<Ext2Error> for vfs::FsError {
    fn from(err: Ext2Error) -> Self {
        match err.inner {
            _ => vfs::FsError::DeviceError,
        }
    }
}

impl core::convert::From<Ext2Error> for Error {
    fn from(err: Ext2Error) -> Self {
        err.inner
    }
}

impl core::convert::From<Error> for Ext2Error {
    fn from(err: Error) -> Self {
        Ext2Error { inner: err }
    }
}

impl core::convert::From<DevError> for Ext2Error {
    fn from(_: DevError) -> Self {
        Ext2Error {
            inner: Error::Other(String::from("unknown")),
        }
    }
}

impl Ext2FileSystem {
    pub fn open(device: Arc<dyn Device>) -> vfs::Result<Arc<Self>> {
        Ok(Self::open_internal(device)?)
    }

    fn open_internal(device: Arc<dyn Device>) -> Result<Arc<Self>, Ext2Error> {
        
        let volume = Ext2Volume { inner: device };
        let fs = Synced::new(volume.clone())?;
        Ok(Arc::new(Ext2FileSystem { inner: fs, volume }))
    }

    pub fn els(&self) {
        for i in self.inner.root_inode().directory().unwrap() {
            let j = i.unwrap();
            
            println!("found inode name!!!{} and id {} type {}", 
                String::from_utf8(j.name).unwrap(),
                j.inode,
                j.ty
            );
            //i.unwrap().name
        }
    }

    pub fn ddd(&self) {
    
        //rcore_fs::vfs::INode
        let file = self.inner.inode_nth(13).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        
        println!("content of file {}", String::from_utf8(buf).unwrap());
    }

    pub fn show_super_block(&self) {
        let u = unsafe { ext2::sys::superblock::Superblock::find(&self.volume).unwrap() };
        println!("{}", String::from_utf8(u.0.last_mnt_path.to_vec()).unwrap());
        let v = unsafe { 
            ext2::sys::block_group::BlockGroupDescriptor::find_descriptor_table(
                &self.volume, Address::new(4, 0), 2
            ).unwrap().0 };
        
        //rcore_fs_sfs::SimpleFileSystem::new_device_inode(&self, device_inode_id, device_inode)
        //println!("{:?}", u.0);
        println!("block size is {}", u.0.block_size());
    }
}

impl FileSystem for Ext2FileSystem {
    fn sync(&self) -> vfs::Result<()> {
        todo!()
    }

    fn root_inode(&self) -> Arc<dyn vfs::INode> {
        todo!()
    }

    fn info(&self) -> vfs::FsInfo {
        todo!()
    }
}

impl Volume<u8, Size512> for Ext2Volume {
    type Error = Ext2Error;

    fn size(&self) -> Size<Size512> {
        Size::Unbounded
    }

    fn commit(&mut self, _slice: Option<VolumeCommit<u8, Size512>>) -> Result<(), Self::Error> {
        unimplemented!()
    }

    unsafe fn slice_unchecked<'a>(
        &'a self,
        range: Range<Address<Size512>>,
    ) -> VolumeSlice<'a, u8, Size512> {
        let index = range.start;
        let len = range.end - range.start;
        let mut vec = vec![0; len.into_index() as usize];
        self.inner
            .read_at(index.into_index() as usize, vec.as_mut_slice())
            .unwrap();
        VolumeSlice::new_owned(vec, index)
    }

    fn slice<'a>(
        &'a self,
        range: Range<Address<Size512>>,
    ) -> Result<VolumeSlice<'a, u8, Size512>, Self::Error> {
        let index = range.start;
        let len = range.end - range.start;
        let mut vec = vec![0; len.into_index() as usize];
        self.inner
            .read_at(index.into_index() as usize, vec.as_mut_slice())?;
        Ok(VolumeSlice::new_owned(vec, index))
    }
}*/


