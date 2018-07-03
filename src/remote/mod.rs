use chunk;
pub mod dropbox;

pub trait Provider {
    fn publish<'a>(&mut self, s: &chunk::Chunk<'a>);
    fn receive(&mut self, h: &chunk::Hash) -> chunk::Data;
}
