use chunk;
use local::{Db, ErrorFind};
use rusqlite;
use crypto::{hash, Hash, HASH_SIZE};

pub struct Sqlite {
    conn: rusqlite::Connection,
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
    fn init(c: &rusqlite::Connection) {
        c.execute_batch(concat!(
            "CREATE TABLE IF NOT EXISTS files (fname TEXT, id INTEGER, fsize INTEGER, PRIMARY KEY(id), CONSTRAINT fname_unique UNIQUE (fname));",
            "CREATE TABLE IF NOT EXISTS hashes (hash BLOB, id INTEGER, idx INTEGER, FOREIGN KEY(id) REFERENCES files(id), PRIMARY KEY(id, idx));")
        ).unwrap();
    }

    pub fn new(dbfile: &str) -> Sqlite {
        let c = rusqlite::Connection::open(dbfile).unwrap();
        Sqlite::init(&c);
        Sqlite { conn: c }
    }
}

impl Db for Sqlite {
    fn save(&mut self, fname: &str, s: &[u8]) -> chunk::Chunks {
        self.conn
            .execute(
                "INSERT INTO files VALUES(?, NULL, ?)",
                &[&fname, &(s.len() as i64)],
            )
            .unwrap();
        let id: i64 = self.conn
            .query_row("SELECT id FROM files WHERE fname=?", &[&fname], |row| {
                row.get(0)
            })
            .unwrap();
        s.chunks(chunk::CHUNK_SIZE)
            .enumerate()
            .map(|(idx, c)| {
                let mut block = [0u8; chunk::CHUNK_SIZE];
                c.iter().enumerate().for_each(|(i, c)| block[i] = *c);
                let block = block;
                let h = hash(&block);
                self.conn
                    .execute(
                        "INSERT INTO hashes VALUES(?, ?, ?)",
                        &[
                            &h.hash().into_iter().cloned().collect::<Vec<u8>>(),
                            &id,
                            &(idx as i64),
                        ],
                    )
                    .unwrap();
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
        let vec: Vec<Hash> = file_info
            .query_map(&[&fname], |row| {
                let mut arr = [0u8; HASH_SIZE];
                row.get::<_, Vec<u8>>(0).into_iter().enumerate().for_each(
                    |(i, x)| {
                        arr[i] = x
                    },
                );
                arr
            })
            .unwrap()
            .map(|x| Hash::new(x.unwrap()))
            .collect();
        if vec.is_empty() {
            Err(ErrorFind::NoMatch)
        } else {
            let file_size: i64 = self.conn
                .query_row("SELECT fsize FROM files WHERE fname=?", &[&fname], |row| {
                    row.get(0)
                })
                .unwrap();
            Ok((file_size as usize, vec))
        }
    }

    fn clean(&mut self, fname: &str) {
        self.conn
            .execute(
                "DELETE FROM hashes WHERE hashes.id=(SELECT id FROM files WHERE fname=?)",
                &[&fname],
            )
            .unwrap();
        self.conn
            .execute("DELETE FROM files WHERE fname=?", &[&fname])
            .unwrap();
    }

    fn list(&mut self) -> Vec<(String, i64)> {
        let mut elems = self.conn
            .prepare("SELECT fname, fsize FROM files ORDER BY fname")
            .unwrap();
        let elems: Vec<_> = elems
            .query_map(&[], |row| (row.get::<_, String>(0), row.get::<_, i64>(1)))
            .unwrap()
            .map(|x| x.unwrap())
            .collect();
        elems
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
        use rusqlite::Connection;
        let c = Connection::open_in_memory().unwrap();
        Sqlite::init(&c);
        Sqlite { conn: c }
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

    #[test]
    fn save_list() {
        let mut s = init();
        let g = || random_blob(chunk::CHUNK_SIZE);
        let f = ["f1", "f2", "f3"];
        let _ = (s.save(f[0], &g()), s.save(f[2], &g()), s.save(f[1], &g()));
        let list = s.list();
        assert_eq!(list.len(), 3);
        list.iter().enumerate().for_each(
            |(i, l)| assert_eq!(f[i], l.0),
        );
    }
}
