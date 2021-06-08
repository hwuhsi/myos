use super::Driver;

pub trait BlockDriver: Driver{
    fn read_at(&self, block_id: usize, buf: &mut [u8]);
    fn write_at(&self, block_id: usize, buf: &[u8]);
}