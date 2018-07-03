use chunk;
use remote::Provider;

#[derive(Debug)]
pub struct Dropbox {
    token: String,
}

impl<'a> Dropbox {
    pub fn new(s: String) -> Dropbox {
        Dropbox { token: s }
    }

    pub fn token(&'a self) -> &'a str {
        return &self.token;
    }
}

impl Provider for Dropbox {
    fn publish<'a>(&mut self, s: &chunk::Chunk<'a>) {
        unimplemented!();
    }

    fn receive(&mut self, h: &chunk::Hash) -> chunk::Data {
        unimplemented!();
    }
}
