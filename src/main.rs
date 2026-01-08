mod app;
mod core;

use std::net::SocketAddr;
use app::server::Server;

fn main() {
    let addr: SocketAddr = "127.0.0.1:10000".parse().unwrap();
    Server::start(addr);
}
