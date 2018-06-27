pub const CHUNK_SIZE: usize = 512;
pub type Hash = [u32; 8];
pub type Data = [u8; CHUNK_SIZE];

pub struct Chunk<'a> {
    hash: Hash,
    chunk: &'a Data,
}

pub type Chunks<'a> = Vec<Chunk<'a>>;
