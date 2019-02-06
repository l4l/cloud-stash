use std::collections::HashMap;

use crate::chunk;
use crate::crypto::{hash, Hash};
use crate::local::{Db, ErrorFind};

#[derive(Default)]
pub struct FileInfo {
    hashes: Vec<Hash>,
    size: usize,
}

pub struct Memory {
    // fname -> [Hash, offset]
    map: HashMap<String, FileInfo>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            map: Default::default(),
        }
    }
}

impl Db for Memory {
    fn save(&mut self, fname: &str, s: &[u8]) -> chunk::Chunks {
        let v = self
            .map
            .entry(fname.to_string())
            .or_insert_with(|| Default::default());
        v.size = s.len();
        s.chunks(chunk::CHUNK_SIZE)
            .enumerate()
            .map(|(idx, c)| {
                let mut block = [0u8; chunk::CHUNK_SIZE];
                c.iter().enumerate().for_each(|(i, c)| block[i] = *c);
                let block = block;
                let h = hash(&block);
                v.hashes.push(h.clone());
                chunk::Chunk {
                    hash: h,
                    chunk: block,
                    idx: idx as u64,
                }
            })
            .collect()
    }

    fn find(&mut self, fname: &str) -> Result<(usize, Vec<Hash>), ErrorFind> {
        self.map
            .get(fname)
            .map(|FileInfo { hashes, size }| (*size, hashes.clone()))
            .ok_or(ErrorFind::NoMatch)
    }

    fn clean(&mut self, fname: &str) {
        if let Some(_) = self.map.remove(fname) {}
    }

    fn list(&mut self) -> Vec<(String, usize)> {
        self.map
            .iter()
            .map(|(name, FileInfo { size, .. })| (name.clone(), size.clone()))
            .collect()
    }
}
