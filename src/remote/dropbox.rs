use chunk;
use crypto::Hash;
use remote::Provider;
use reqwest;
use reqwest::header::{HeaderName, CONNECTION, CONTENT_TYPE};
use serde_json;

#[derive(Debug)]
pub struct Dropbox {
    token: String,
}

impl<'a> Dropbox {
    pub fn new(s: String) -> Dropbox {
        Dropbox { token: s }
    }

    pub fn token(&'a self) -> &'a str {
        &self.token
    }
}

#[derive(Clone)]
struct DropboxApiArg {
    val: serde_json::Value,
}

const DROPBOX_HDR: &str = "Dropbox-API-Arg";

impl Provider for Dropbox {
    fn publish(&mut self, s: &chunk::Chunk) {
        let client = reqwest::Client::new();
        let res = client
            .post("https://content.dropboxapi.com/2/files/upload")
            .bearer_auth(self.token().to_owned())
            .header(
                HeaderName::from_static(DROPBOX_HDR),
                json!({"path": format!("/{}", &s.hash),
                            "mode": "add",
                            "autorename": false})
                .to_string(),
            )
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(s.chunk.to_vec())
            .send()
            .unwrap();
        debug!("{:?}", res);
    }

    fn receive(&mut self, h: &Hash) -> chunk::Data {
        let client = reqwest::Client::new();
        let mut res = client
            .post("https://content.dropboxapi.com/2/files/download")
            .bearer_auth(self.token().to_owned())
            .header(
                HeaderName::from_static(DROPBOX_HDR),
                json!({ "path": format!("/{}", &h) }).to_string(),
            )
            .header(CONNECTION, "close")
            .send()
            .unwrap();
        debug!("{:?}", res);
        let mut buf = Vec::new();
        assert_eq!(res.copy_to(&mut buf).unwrap(), chunk::CHUNK_SIZE as u64);
        let mut r = [0u8; chunk::CHUNK_SIZE];
        r.copy_from_slice(&buf[..chunk::CHUNK_SIZE]);
        r
    }

    fn delete(&mut self, hs: &[Hash]) {
        let client = reqwest::Client::new();
        let res = client
            .post("https://content.dropboxapi.com/2/files/download")
            .bearer_auth(self.token().to_owned())
            .header(
                HeaderName::from_static(DROPBOX_HDR),
                json!({
                    "entries": serde_json::Value::Array(
                        hs.iter()
                          .map(|h| json!({"path": format!("/{}", &h)})).collect())})
                .to_string(),
            )
            .send()
            .unwrap();
        debug!("{:?}", res);
    }
}
