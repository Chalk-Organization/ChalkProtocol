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
    /// cilent.connect_to("127.0.0.1:8080").unwrap();
    /// ```
    pub fn connect_to<T: ToSocketAddrs>(&mut self, addr: T) -> Result<&mut Self, String> {
        self.streams.push(
            TcpStream::connect(addr)
                // Checks if there is an error an if so then map the error to a custom error
                .map_err(|x| -> String { format!("Failed to Connect to IP Address: `{}`", x) })?,
        );
        Ok(self)
    }
}
