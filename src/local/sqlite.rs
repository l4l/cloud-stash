use chunk;
use local::{Db, ErrorFind};

pub struct Sqlite {}

impl Db for Sqlite {
    fn save<'a>(&mut self, _s: &'a [u8]) -> chunk::Chunks<'a> {
        unimplemented!();
    }
    fn find(&mut self, _fname: &str) -> Result<Vec<chunk::Hash>, ErrorFind> {
        unimplemented!();
    }
}
