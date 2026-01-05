mod app;
mod net;
use crate::app::server::Server; 

fn main(){
    Server::start(); 
}