use chunk;
use local::{Db, ErrorFind};
use sqlite;
use crypto::{hash, Hash, HASH_SIZE};

pub struct Sqlite {
    conn: sqlite::Connection,
}

impl Sqlite {
    /// # Relational schema
    ///
    /// ## Table files
    /// Maps unique filename to its unique identifier
    ///
    /// ## Table hashes
    /// Maps unique pair of chunk hash and chunks' file id to its positional index in file
    ///
    pub fn new(dbfile: &str) -> Sqlite {
        let c = sqlite::open(dbfile).unwrap();
        c.execute(concat!(
            "CREATE TABLE IF NOT EXISTS files (fname TEXT, id INTEGER, fsize INTEGER, PRIMARY KEY(id), CONSTRAINT fname_unique UNIQUE (fname));",
            "CREATE TABLE IF NOT EXISTS hashes (hash BLOB, id INTEGER, idx INTEGER, FOREIGN KEY(id) REFERENCES files(id), PRIMARY KEY(id, idx));",
        )).unwrap();
        Sqlite { conn: c }
    }
}

impl Db for Sqlite {
    fn save<'a>(&mut self, fname: &str, s: &'a [u8]) -> chunk::Chunks {
        let mut file_add = self.conn
            .prepare("INSERT INTO files VALUES(?, NULL, ?)")
            .unwrap();
        file_add.bind(1, fname).unwrap();
        file_add.bind(2, s.len() as i64).unwrap();
        assert_eq!(file_add.next().unwrap(), sqlite::State::Done);
        let mut get_id = self.conn
            .prepare("SELECT id FROM files WHERE fname=?")
            .unwrap();
        get_id.bind(1, fname).unwrap();
        get_id.next().unwrap();
        let id = get_id.read::<i64>(0).unwrap();
        s.chunks(chunk::CHUNK_SIZE)
            .enumerate()
            .map(|(idx, c)| {
                let mut block = [0u8; chunk::CHUNK_SIZE];
                c.iter().enumerate().for_each(|(i, c)| block[i] = *c);
                let block = block;
                let h = hash(&block);
                let mut add_chunk = self.conn
                    .prepare("INSERT INTO hashes VALUES(?, ?, ?)")
                    .unwrap();
                add_chunk.bind(1, h.hash() as &[u8]).unwrap();
                add_chunk.bind(2, id).unwrap();
                add_chunk.bind(3, idx as i64).unwrap();
                assert_eq!(add_chunk.next().unwrap(), sqlite::State::Done);
                chunk::Chunk {
                    hash: h,
                    chunk: block,
                    idx: idx as u64,
                }
            })
            .collect()
    }

    fn find(&mut self, fname: &str) -> Result<(usize, Vec<Hash>), ErrorFind> {
        let mut file_info = self.conn
            .prepare(
                "SELECT hash, idx FROM hashes WHERE hashes.id=(SELECT id FROM files WHERE fname=?) ORDER BY idx",
            )
            .unwrap();
        file_info.bind(1, fname).unwrap();
        let mut vec = Vec::new();
        while let sqlite::State::Row = file_info.next().unwrap() {
            let hash_blob = file_info.read::<Vec<u8>>(0).unwrap();
            let mut hash = [0u8; HASH_SIZE];
            hash_blob.iter().enumerate().for_each(|(i, h)| hash[i] = *h);
            vec.push(Hash::new(hash));
        }
        if vec.is_empty() {
            Err(ErrorFind::NoMatch)
        } else {
            let mut file_size = self.conn
                .prepare("SELECT fsize FROM files WHERE fname=?")
                .unwrap();
            file_size.bind(1, fname).unwrap();
            file_size.next().unwrap();
            Ok((file_size.read::<i64>(0).unwrap() as usize, vec))
        }
    }

    fn clean(&mut self, fname: &str) {
        let exec = |qry| {
            let mut rmv = self.conn.prepare(qry).unwrap();
            rmv.bind(1, fname).unwrap();
            rmv.next().unwrap();
        };
        exec(
            "DELETE FROM hashes WHERE hashes.id=(SELECT id FROM files WHERE fname=?)",
        );
        exec("DELETE FROM files WHERE fname=?");
    }
}


#[cfg(test)]
mod test {
    extern crate rand;
    use local::sqlite::Sqlite;
    use local::Db;
    use chunk;
    use crypto;

    fn init() -> Sqlite {
        Sqlite::new(":memory:")
    }

    fn random_blob(sz: usize) -> Vec<u8> {
        (0..sz).map(|_| rand::random()).collect()
    }

    #[test]
    fn save_chunks_properly() {
        let b = random_blob(chunk::CHUNK_SIZE * 3 + 1);
        let chunks = init().save("myfile", &b);
        assert_eq!(chunks.len(), 4);
        for i in 0..4 {
            assert_eq!(chunks[i].idx, i as u64);
            assert_eq!(&crypto::hash(&chunks[i].chunk), &chunks[i].hash);
        }
        chunks
            .iter()
            .flat_map(|c| c.chunk.iter())
            .zip(&b)
            .for_each(|(c, b)| assert_eq!(c, b));
    }

    #[test]
    fn find_saved() {
        let mut s = init();
        let b = random_blob(chunk::CHUNK_SIZE * 3 + 1);
        let fname = "myfile";
        let chunks = s.save(fname, &b);
        let (bsize, hashes) = s.find(fname).unwrap();
        assert_eq!(bsize, b.len());
        chunks.iter().map(|c| &c.hash).zip(hashes).for_each(
            |(c, h)| {
                assert_eq!(c, &h)
            },
        );
    }

    #[test]
    fn save_same_chunks() {
        let b = random_blob(chunk::CHUNK_SIZE * 3 + 1);
        let mut s = init();
        let first_chunks = s.save("myfile1", &b);
        let second_chunks = s.save("myfile2", &b);
        first_chunks
            .iter()
            .flat_map(|c| c.chunk.iter())
            .zip(second_chunks.iter().flat_map(|c| c.chunk.iter()))
            .zip(&b)
            .for_each(|((c1, c2), b)| {
                assert_eq!(c1, b);
                assert_eq!(c2, b);
            });
    }

    #[test]
    fn save_remove_find() {
        let mut s = init();
        let b = random_blob(chunk::CHUNK_SIZE * 3 + 1);
        let fname = "myfile";
        let _ = s.save(fname, &b);
        assert!(s.find(fname).is_ok());
        s.clean(fname);
        assert!(s.find(fname).is_err());
    }
}
