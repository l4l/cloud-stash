use crate::chunk;
use crate::crypto::Hash;

pub mod dropbox;

pub trait Provider {
    fn publish(&mut self, s: &chunk::Chunk);
    fn receive(&mut self, h: &Hash) -> chunk::Data;
    fn delete(&mut self, hs: &[Hash]);
}
