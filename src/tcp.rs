use std::net::{TcpStream, ToSocketAddrs};

pub struct TCPClient {
    streams: Vec<TcpStream>,
}

impl TCPClient {
    pub fn new() -> Self {
        Self { streams: vec![] }
    }

    /// Connects to an IP Address given.
    ///
    /// # Example
    /// ```
    /// let mut client = TCPClient::new();
    /// cilent.connect_to("127.0.0.1:8080");
    /// ```
    pub fn connect_to<T: ToSocketAddrs>(&mut self, addr: T) -> &mut Self {
        self.streams
            .push(TcpStream::connect(addr).expect("Failed to Connect to IP Address"));
        self
    }
}
