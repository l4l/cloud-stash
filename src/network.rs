use std::fs::File;
use std::io::Read;
use {local, remote};

pub fn upload(file: &str) {
    let mut content = Vec::new();
    File::open(&file)
        .expect(&format!("Can't open {}", &file))
        .read_to_end(&mut content)
        .expect("Something happened during file reading");

    let chunks = local::save(&content);
    for c in chunks {
        remote::publish(&c);
    }
}

pub fn download(fname: &str) {
    let hash_list = local::find(&fname).unwrap();
    for h in hash_list {
        remote::receive(&h);
    }
}
