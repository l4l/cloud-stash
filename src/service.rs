use std::cmp::min;
use std::fs::File;
use std::io::{Read, Write};

use crate::chunk::CHUNK_SIZE;
use crate::{local, remote};

pub struct Service<Db, Provider> {
    pub db: Db,
    pub provider: Provider,
}

impl<Db: local::Db, Provider: remote::Provider> Service<Db, Provider> {
    // TODO?: return result
    pub fn upload(&mut self, fname: &str, file: &str) {
        let mut content = Vec::new();
        File::open(&file)
            .unwrap_or_else(|_| panic!("Can't open {}", &file))
            .read_to_end(&mut content)
            .expect("Something happened during file reading");
        let chunks = self.db.save(fname, &content);
        for c in chunks {
            self.provider.publish(&c);
        }
    }

    // TODO?: return result
    pub fn download(&mut self, fname: &str, newname: &str) {
        let (mut fsize, hash_list) = self.db.find(&fname).unwrap();
        let mut file = File::create(newname).unwrap();
        for h in hash_list {
            file.write_all(&self.provider.receive(&h)[..min(fsize, CHUNK_SIZE)])
                .unwrap();
            fsize -= CHUNK_SIZE;
        }
    }

    pub fn remove(&mut self, fname: &str) {
        let (_, hash_list) = self.db.find(&fname).unwrap();
        self.db.clean(&fname);
        self.provider.delete(&hash_list);
    }
}
