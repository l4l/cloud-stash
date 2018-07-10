extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sqlite;
extern crate sha3;
extern crate reqwest;
#[macro_use]
extern crate serde_json;
extern crate hyper;
extern crate netfuse;
extern crate fuse;
extern crate libc;
extern crate time;

mod local;
mod remote;
mod chunk;
mod service;
mod get_token;
mod crypto;
mod fs;

const USAGE: &str = "
cloud-stash is a tool for managing multiple file storage accounts.
Usage:
  cloud-stash (-a | --auth)
  cloud-stash (-u | --upload) <file> <newname> <token>
  cloud-stash (-d | --download) <file> <newname> <token>
  cloud-stash (-r | --remove) <file> <token>
  cloud-stash (-m | --mount) <file> <token>
  cloud-stash (-h | --help)
  cloud-stash --version

Arguments:
  <file>            File path for working with
  <newname>         New name of the uploaded/saved file
  <token>           Dropbox auth token

Options:
  -a --auth                Authorize app and get a token
  -u --upload              Upload a file
  -d --download            Download a file
  -r --remove              File removing from the remote host
  -m --mount               Perform fs mount
  -h --help                Show this help.
  --version                Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: Option<String>,
    arg_newname: Option<String>,
    arg_token: Option<String>,
    flag_auth: bool,
    flag_upload: bool,
    flag_download: bool,
    flag_remove: bool,
    flag_mount: bool,
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_auth {
        get_token::run_handler();
    }
    let db = local::sqlite::Sqlite::new("db");
    let provider = remote::dropbox::Dropbox::new(args.arg_token.expect(USAGE));
    if args.flag_upload {
        service::Service {
            db: db,
            provider: provider,
        }.upload(
            &args.arg_newname.expect(USAGE),
            &args.arg_file.expect(USAGE),
        );
    } else if args.flag_download {
        service::Service {
            db: db,
            provider: provider,
        }.download(
            &args.arg_file.expect(USAGE),
            &args.arg_newname.expect(USAGE),
        );
    } else if args.flag_remove {
        service::Service {
            db: db,
            provider: provider,
        }.remove(&args.arg_file.expect(USAGE));
    } else if args.flag_mount {
        fs::stashfs::StashFs::mount_with(db, provider, &args.arg_file.expect(USAGE));
    }
    println!("{}", USAGE);
}
