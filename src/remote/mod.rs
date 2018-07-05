use chunk;
use crypto::Hash;
pub mod dropbox;

pub trait Provider {
    fn publish<'a>(&mut self, s: &chunk::Chunk);
    fn receive(&mut self, h: &Hash) -> chunk::Data;
}
