mod local;
mod remote;
mod chunk;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let usage = format!("Use {} <file>", env::args().nth(0).unwrap());
    let file = env::args().nth(1).expect(&usage);

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
