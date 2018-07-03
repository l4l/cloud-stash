use std::fs::File;
use std::io::Read;
use {local, remote};

pub struct Service<Db, Provider> {
    pub db: Db,
    pub provider: Provider,
}

impl<Db: local::Db, Provider: remote::Provider> Service<Db, Provider> {
    // TODO?: return result
    pub fn upload(&mut self, file: &str) {
        let mut content = Vec::new();
        File::open(&file)
            .expect(&format!("Can't open {}", &file))
            .read_to_end(&mut content)
            .expect("Something happened during file reading");
        let chunks = self.db.save(&content);
        for c in chunks {
            self.provider.publish(&c);
        }
    }

    // TODO?: return result
    pub fn download(&mut self, fname: &str) {
        let hash_list = self.db.find(&fname).unwrap();
        for h in hash_list {
            self.provider.receive(&h);
        }
    }
}
