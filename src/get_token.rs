use std::net::TcpListener;
use std::io::Write;
use std::process::exit;

const URL: &str = concat!(
    "https://www.dropbox.com/oauth2/authorize?",
    "response_type=token",
    "&client_id=g71rb26y469u0n6",
    "&redirect_uri=http://localhost:8080"
);
const SCRIPT: &str = concat!(
    "HTTP/1.1 200 OK\r\n",
    "Content-Type: text/html; charset=utf8\r\n",
    "Content-Length: 172\r\n\r\n",
    "<html><body><script type=\"text/javascript\">",
    "var re = /.*access_token=(.*?)&.*/g;",
    "m = re.exec(location.href);",
    "document.write('Your token is: ' + m[1]);",
    "</script></body></html>"
);

pub fn run_handler() {
    println!("App auth: {}", URL);
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream.and_then(|mut s| s.write(SCRIPT.as_bytes())) {
            Ok(_) => {
                exit(0);
            }
            Err(e) => {
                panic!("Something went wrong: {}\n Try again.", e);
            }
        }
    }
}
