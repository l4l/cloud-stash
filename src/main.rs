extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sqlite;
extern crate sha3;

mod local;
mod remote;
mod chunk;
mod service;
mod get_token;
mod crypto;

const USAGE: &'static str = "
cloud-stash is a tool for managing multiple file storage accounts.
Usage:
  cloud-stash (-a | --auth)
  cloud-stash (-u | --upload) <file> <newname> <token>
  cloud-stash (-d | --download) <file> <token>
  cloud-stash (-h | --help)
  cloud-stash --version

Arguments:
  <file>            File path for working with
  <newname>        New name of the uploaded file
  <token>           Dropbox auth token

Options:
  -a --auth              Authorize app and get a token
  -u --upload              Upload a file
  -d --download            Download a file
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
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_auth {
        get_token::run_handler();
    }
    let mut service = service::Service::<local::sqlite::Sqlite, remote::dropbox::Dropbox> {
        db: local::sqlite::Sqlite::new(":memory:"),
        provider: remote::dropbox::Dropbox::new(args.arg_token.expect(USAGE)),
    };
    if args.flag_upload {
        service.upload(
            &args.arg_newname.expect(USAGE),
            &args.arg_file.expect(USAGE),
        );
    } else if args.flag_download {
        service.download(&args.arg_file.expect(USAGE));
    }
}
