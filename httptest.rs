
#[warn(non_camel_case_types, non_uppercase_statics, non_uppercase_statics, unnecessary_qualification, managed_heap_memory)]

extern mod httpserver;

use std::rt::io::net::ip::Ipv4Addr;
use std::rt::io::net::ip::SocketAddr;
use std::rt::io::Writer;

use httpserver::HttpRequest;

fn cb(request: &mut HttpRequest) {
        request.stream.write(request.path.as_bytes());
}

fn cbfactory() -> ~fn(request: &mut HttpRequest) {
        |request| cb(request)
}

fn main() {
        let addr = ~SocketAddr {ip: Ipv4Addr(127, 0, 0, 1), port: 8000};
        let server = httpserver::HttpServer::new(addr);
        server.run(cbfactory);
}
