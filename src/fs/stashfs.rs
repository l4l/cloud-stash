use netfuse::{NetworkFilesystem, LibcError, Metadata, DirEntry, MountOptions, mount};
use local::Db;
use remote::Provider;
use std::path::Path;
use libc;
use fuse::FileType;
use time::Timespec;
use std::cmp::min;
use chunk::CHUNK_SIZE;
use crypto::Hash;

pub struct StashFs<D: Db, P: Provider> {
    db: D,
    provider: P,
}

fn get_path(path: &Path) -> Result<&str, LibcError> {
    path.to_str()
        .map(|s| &s[1..])
        .and_then(|s| match s.find(|x| x == '/') {
            None => Some(s),
            _ => None,
        })
        .ok_or(libc::ENOENT)
}

impl<D: Db, P: Provider> StashFs<D, P> {
    fn find(&mut self, fname: &str) -> Result<(usize, Vec<Hash>), LibcError> {
        self.db.find(fname).map_err(|_| libc::ENOENT)
    }

    pub fn mount_with(d: D, p: P, path: &str) {
        mount(
            StashFs { db: d, provider: p },
            MountOptions::new(&Path::new(path)),
        )
    }
}

impl<D: Db, P: Provider> NetworkFilesystem for StashFs<D, P> {
    fn lookup(&mut self, path: &Path) -> Result<Metadata, LibcError> {
        println!("#lookup {:?}", path);
        let (fsize, _) = get_path(path).and_then(|p| self.find(p))?;
        Ok(Metadata {
            size: fsize as u64,
            atime: Timespec::new(0, 0),
            mtime: Timespec::new(0, 0),
            ctime: Timespec::new(0, 0),
            crtime: Timespec::new(0, 0),
            kind: FileType::RegularFile,
            perm: 0o777,
        })
    }

    fn read(&mut self, path: &Path, buffer: &mut Vec<u8>) -> Result<usize, LibcError> {
        println!("#read {:?}", path);
        let (fsize, hlist) = get_path(path).and_then(|p| self.find(p))?;
        buffer.reserve(fsize);
        hlist.iter().enumerate().for_each(|(i, h)| {
            buffer.extend_from_slice(
                &self.provider.receive(&h)[..min(fsize - i * CHUNK_SIZE, CHUNK_SIZE)],
            )
        });
        Ok(fsize)
    }

    fn write(&mut self, path: &Path, data: &[u8]) -> Result<(), LibcError> {
        println!("#write {:?}", path);
        let fname = get_path(path)?;
        // make sure the file exists
        if let Ok((_, hs)) = self.find(&fname) {
            // TODO: full cleaning shouldn't be done every time
            self.db.clean(&fname);
            self.provider.delete(&hs);
        }
        self.db.save(fname, data).iter().for_each(
            |c| self.provider.publish(c),
        );
        Ok(())
    }

    fn unlink(&mut self, path: &Path) -> Result<(), LibcError> {
        println!("#unlink {:?}", path);
        let fname = get_path(path)?;
        let (_, hs) = self.find(&fname)?;
        self.db.clean(&fname);
        // TODO: use counter to check whether it should be deleted
        self.provider.delete(&hs);
        Ok(())
    }

    fn readdir(&mut self, path: &Path) -> Vec<Result<DirEntry, LibcError>> {
        let begin = match get_path(path) {
            Ok(s) => s,
            Err(e) => return vec![Err(e)],
        };
        self.db
            .list()
            .into_iter()
            .inspect(|(s, meta)| println!("{:?}", (s, meta)))
            .filter(|(s, _)| s.as_str().starts_with(&begin))
            .map(|(mut s, meta)| {s.drain(begin.len()..).fold(0, |acc, _| acc); (s, meta)})
            // .filter(|(s, _)| s.find('/').is_none())
            .map(|(s, meta)| {
                Ok(DirEntry::new(
                    ::std::ffi::OsString::from(s),
                    Metadata {
                        size: meta as u64,
                        atime: Timespec::new(0, 0),
                        mtime: Timespec::new(0, 0),
                        ctime: Timespec::new(0, 0),
                        crtime: Timespec::new(0, 0),
                        kind: FileType::RegularFile,
                        perm: 0o777,
                    },
                ))
            })
            .collect()
    }
}
