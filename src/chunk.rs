use crypto::Hash;
pub const CHUNK_SIZE: usize = 512;
pub type Data = [u8; CHUNK_SIZE];

pub struct Chunk {
    pub hash: Hash,
    pub chunk: Data,
    pub idx: u64,
}

pub type Chunks = Vec<Chunk>;
