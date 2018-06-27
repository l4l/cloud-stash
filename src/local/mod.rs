use chunk;

pub fn save<'a>(_s: &'a [u8]) -> chunk::Chunks<'a> {
    unimplemented!();
}

#[derive(Debug)]
pub enum ErrorFind {
    NoMatch,
    Matched(Vec<String>),
}

pub fn find(_fname: &String) -> Result<Vec<chunk::Hash>, ErrorFind> {
    unimplemented!();
}
