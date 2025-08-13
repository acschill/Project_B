
use anyhow::Result;
pub type BlobPtr = u64;
pub struct KvStore;
impl KvStore {
    pub fn put_blob(&mut self, _ptr: BlobPtr, _data: &[u8]) -> Result<()> { Ok(()) }
    pub fn get_blob(&self, _ptr: BlobPtr) -> Result<Vec<u8>> { Ok(Vec::new()) }
}
