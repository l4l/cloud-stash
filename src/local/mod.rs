use crate::chunk;
use crate::crypto::Hash;

pub mod memory;
#[cfg(feature = "persistent")]
pub mod sqlite;

#[derive(Debug)]
pub enum ErrorFind {
    /// File cannot be found
    NoMatch,
}

pub trait Db {
    fn save(&mut self, fname: &str, s: &[u8]) -> chunk::Chunks;
    // TODO: replace usize with metainfo
    fn find(&mut self, fname: &str) -> Result<(usize, Vec<Hash>), ErrorFind>;
    fn clean(&mut self, fname: &str);
    fn list(&mut self) -> Vec<(String, usize)>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::Hash;

    fn test_save_and_find<D: Db, F: FnOnce() -> D>(f: F) {
        let mut mem = f();
        let buf = ([1, 2, 3, 4, 5], [0, 0, 0, 0], [6, 5, 4, 3, 2, 1]);
        mem.save("file1", &buf.0);
        mem.save("file2", &buf.1);
        mem.save("file3", &buf.2);

        let size1 = buf.0.len();
        let size2 = buf.1.len();
        let size3 = buf.2.len();
        assert_eq!(
            mem.find("file1").unwrap(),
            (
                size1,
                vec![Hash::new([
                    138, 239, 195, 139, 37, 51, 206, 224, 1, 48, 66, 167, 28, 54, 154, 4, 182, 89,
                    6, 101, 37, 91, 120, 227, 245, 217, 14, 16, 169, 20, 236, 73
                ])]
            )
        );
        assert_eq!(
            mem.find("file2").unwrap(),
            (
                size2,
                vec![Hash::new([
                    98, 45, 225, 225, 86, 141, 222, 243, 108, 75, 137, 183, 6, 176, 82, 1, 193, 52,
                    129, 195, 87, 93, 15, 200, 4, 255, 130, 36, 120, 127, 203, 89
                ])]
            )
        );
        assert_eq!(
            mem.find("file3").unwrap(),
            (
                size3,
                vec![Hash::new([
                    2, 147, 59, 163, 101, 135, 58, 137, 86, 67, 119, 88, 233, 235, 154, 57, 136,
                    100, 119, 5, 98, 25, 173, 141, 237, 30, 4, 195, 252, 89, 139, 113
                ])]
            )
        );
    }

    #[test]
    fn test_sqlite_save_and_find() {
        use memory::Memory;
        test_save_and_find::<Memory, _>(Memory::new);
    }

    #[test]
    #[cfg(feature = "persistent")]
    fn test_memory_save_and_find() {
        use sqlite::Sqlite;
        test_save_and_find::<Sqlite, _>(|| Sqlite::new("test.db"));
        std::fs::remove_file("test.db").unwrap();
    }
}
