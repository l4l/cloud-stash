extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod local;
mod remote;
mod chunk;
mod network;

const USAGE: &'static str = "
cloud-stash is a tool for managing multiple file storage accounts.
Usage:
  cloud-stash (-u | --upload) <file>
  cloud-stash (-d | --download) <file>
  cloud-stash (-h | --help)
  cloud-stash --version

Arguments:
  <file>            File path for working with

Options:
  -u --upload              Upload a file
  -d --download           Download a file
  -h --help                Show this help.
  --version                Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: Option<String>,
    flag_upload: bool,
    flag_download: bool,
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_upload {
        network::upload(&args.arg_file.expect(USAGE));
    } else if args.flag_download {
        network::download(&args.arg_file.expect(USAGE));
    }
}
