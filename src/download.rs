mod local;
mod remote;
mod chunk;

use std::env;

fn main() {
    let usage = format!("Use {} <filename>", env::args().nth(0).unwrap());
    let fname = env::args().nth(1).expect(&usage);

    let hash_list = local::find(&fname).unwrap();
    for h in hash_list {
        remote::receive(&h);
    }
}
