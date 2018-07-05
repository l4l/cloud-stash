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
            "CREATE TABLE IF NOT EXISTS files (fname TEXT, id INTEGER, PRIMARY KEY(id), CONSTRAINT fname_unique UNIQUE (fname));",
            "CREATE TABLE IF NOT EXISTS hashes (hash BLOB, id INTEGER, idx INTEGER, FOREIGN KEY(id) REFERENCES files(id), PRIMARY KEY(hash, id));",
        )).unwrap();
        Sqlite { conn: c }
    }
}

impl Db for Sqlite {
    fn save<'a>(&mut self, s: &'a [u8]) -> chunk::Chunks {
        let fname = hash(s);
        let mut file_add = self.conn
            .prepare("INSERT INTO files VALUES(?, NULL)")
            .unwrap();
        file_add.bind(1, fname.iter().as_slice()).unwrap();
        assert_eq!(file_add.next().unwrap(), sqlite::State::Done);
        let mut get_id = self.conn
            .prepare("SELECT id FROM files WHERE fname=?")
            .unwrap();
        get_id.bind(1, fname.iter().as_slice()).unwrap();
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
                add_chunk.bind(1, h.iter().as_slice()).unwrap();
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

    fn find(&mut self, fname: &str) -> Result<Vec<Hash>, ErrorFind> {
        let mut file_info = self.conn
            .prepare(
                "SELECT hash, idx FROM hashes WHERE hashes.id=(SELECT id FROM files WHERE fname=?)",
            )
            .unwrap();
        file_info.bind(1, fname).unwrap();
        let mut vec = Vec::new();
        while let sqlite::State::Row = file_info.next().unwrap() {
            let hash_blob = file_info.read::<Vec<u8>>(0).unwrap();
            let mut hash = [0u8; HASH_SIZE];
            hash_blob.iter().enumerate().for_each(|(i, h)| hash[i] = *h);
            vec.push(hash);
        }
        if vec.len() == 0 {
            Err(ErrorFind::NoMatch)
        } else {
            Ok(vec)
        }
    }
}
