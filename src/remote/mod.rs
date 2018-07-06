use chunk;
use crypto::{Hash, Hashes};
pub mod dropbox;

pub trait Provider {
    fn publish<'a>(&mut self, s: &chunk::Chunk);
    fn receive(&mut self, h: &Hash) -> chunk::Data;
    fn delete(&mut self, hs: &Hashes);
}
