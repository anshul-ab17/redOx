use mio::Poll;

pub struct Server;

impl Server {
    pub fn start(){
        let _poll =Poll::new().expect("Poll failed");
        print!("server booting!")
    }
}
