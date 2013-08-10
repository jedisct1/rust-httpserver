
#[link(name="httpserver", vers="0.1")];
#[crate_type="lib"];
#[warn(non_camel_case_types, non_uppercase_statics, non_uppercase_statics, unnecessary_qualification, managed_heap_memory)]

use std::cell::Cell;
use std::rt::io::net::tcp::TcpListener;
use std::rt::io::net::tcp::TcpStream;
use std::rt::io::net::ip::SocketAddr;
use std::rt::io::{Reader, Writer, Listener};
use std::str;

static MAX_HEADER_LEN: uint = 8192;

enum HttpMethod {
        NONE, GET, HEAD
}

struct HttpRequest {
        method: HttpMethod,
        path: ~str,
        stream: ~TcpStream
}

impl HttpRequest {
        pub fn new(stream: ~TcpStream) -> HttpRequest {
                HttpRequest { method: NONE, path: ~"", stream: stream }
        }
        
        pub fn process_header(&self, header: &str) {
                let sh: ~[&str] = header.split_iter(':').collect();
                if sh.len() != 2 {
                        return;
                }
                let (_key, _value) = (sh[0].trim(), sh[1].trim());
        }

        pub fn process_query(&mut self, query: &str) {
                let sh: ~[&str] = query.split_iter(' ').collect();
                if sh.len() != 3 {
                        return;
                }
                let (method, path, _protocol) = (sh[0], sh[1], sh[2]);
                match method {
                        "GET"  =>   self.method = GET,
                        "HEAD" =>   self.method = HEAD,
                        _      => { self.method = NONE; return }
                }
                self.path = path.to_owned();
        }

        pub fn process(&mut self, cb: &fn(request: &mut HttpRequest)) {
                let mut buf = ~[0u8, ..MAX_HEADER_LEN];
                self.stream.read(buf);
                let headersbuf = ~str::from_bytes(buf);
                let mut headers = headersbuf.any_line_iter();
                let query = match headers.next() {
                        None => return,
                        Some(query) => query,
                };
                self.process_query(query);
                for header in headers {
                        self.process_header(header);
                };
                self.stream.write(bytes!("Server: RustyPing\r\n"));
                self.stream.write(bytes!("Content-Type: text/plain\r\n"));
                self.stream.write(bytes!("Connection: closed\r\n"));
                self.stream.write(bytes!("\r\n"));
                cb(self);
        }
}

struct HttpServer<'self> {
        addr: &'self SocketAddr
}

impl<'self> HttpServer<'self> {
        pub fn new<'r>(addr: &'r SocketAddr) -> HttpServer<'r> {
                HttpServer { addr: addr }
        }

        pub fn run(&self, cbfactory: &fn() -> ~fn(request: &mut HttpRequest)) {
                let mut listener = TcpListener::bind(*self.addr).unwrap();
                println("listener is ready");
                loop {
                        let connection = listener.accept();
                        let stream =
                                match connection {
                                Some(stream) => Cell::new(stream),
                                None => break
                        };
                        let cb = cbfactory();
                        do spawn {
                                let stream = ~stream.take();
                                let mut request = HttpRequest::new(stream);
                                do request.process |httprequest| {
                                        cb(httprequest);
                                };
                        }
                }                
        }
}
