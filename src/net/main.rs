use std::net::SocketAddr;
use redux::app::server::Server;

fn main() {
    let addr: SocketAddr = "127.0.0.1:10000".parse().unwrap();
    Server::start(addr);
}
