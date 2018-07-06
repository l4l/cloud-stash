use std;
use chunk;
use crypto::Hash;
use remote::Provider;
use reqwest;
use hyper::error::Error;
use reqwest::header::{Authorization, Header, Bearer, Formatter, Raw, ContentType};
use serde_json;
use crypto::hash_hex;

#[derive(Debug)]
pub struct Dropbox {
    token: String,
}

impl<'a> Dropbox {
    pub fn new(s: String) -> Dropbox {
        Dropbox { token: s }
    }

    pub fn token(&'a self) -> &'a str {
        return &self.token;
    }
}

#[derive(Clone)]
struct DropboxApiArg {
    val: serde_json::Value,
}

impl Header for DropboxApiArg {
    fn header_name() -> &'static str {
        "Dropbox-API-Arg"
    }

    fn parse_header(raw: &Raw) -> Result<Self, Error> {
        Ok(DropboxApiArg {
            val: raw.one()
                .and_then(|l| serde_json::from_slice(l).ok())
                .ok_or(Error::Header)?,
        })
    }
    fn fmt_header(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.fmt_line(&self.val)
    }
}

impl Provider for Dropbox {
    fn publish<'a>(&mut self, s: &chunk::Chunk) {
        let client = reqwest::Client::new();
        let res = client
            .post("https://content.dropboxapi.com/2/files/upload")
            .header(Authorization(Bearer { token: self.token().to_owned() }))
            .header(DropboxApiArg {
                val: json!({"path": format!("/{}", &hash_hex(&s.hash)),
                            "mode": "add",
                            "autorename": false}),
            })
            .header(ContentType::octet_stream())
            .body(s.chunk.iter().map(|x| *x).collect::<Vec<u8>>())
            .send()
            .unwrap();
        println!("{:?}", res);
    }

    fn receive(&mut self, h: &Hash) -> chunk::Data {
        let client = reqwest::Client::new();
        let mut res = client
            .post("https://content.dropboxapi.com/2/files/download")
            .header(Authorization(Bearer { token: self.token().to_owned() }))
            .header(DropboxApiArg {
                val: json!({"path": format!("/{}", &hash_hex(&h))}),
            })
            .send()
            .unwrap();
        let mut buf = Vec::new();
        res.copy_to(&mut buf).unwrap();
        assert_eq!(buf.len(), chunk::CHUNK_SIZE);
        println!("{:?}", res);
        let mut r = [0u8; chunk::CHUNK_SIZE];
        r.copy_from_slice(&buf[..chunk::CHUNK_SIZE]);
        r
    }
}
