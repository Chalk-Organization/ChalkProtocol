use std::net::{TcpStream, ToSocketAddrs};

pub struct TCPClient {
    streams: Vec<TcpStream>,
}

impl TCPClient {
    pub fn new() -> Self {
        Self { streams: vec![] }
    }

    pub fn connect_to<T: ToSocketAddrs>(&mut self, addr: T) -> &mut Self {
        self.streams
            .push(TcpStream::connect(addr).expect("Failed to Connect to IP Address"));
        self
    }
}
