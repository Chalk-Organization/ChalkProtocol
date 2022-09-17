use std::net::{TcpStream, ToSocketAddrs};

pub struct TCPClient {
    streams: Vec<TcpStream>,
}

impl TCPClient {
    pub fn new() -> Self {
        Self { streams: vec![] }
    }

    // Inner version of `connect_to` to prevent code duplication
    fn connect_to_inner<T: ToSocketAddrs>(&mut self, addr: T) -> Result<&mut Self, String> {
        self.streams.push(
            TcpStream::connect(addr)
                // Checks if there is an error an if so then map the error to a custom error
                .map_err(|x| -> String { format!("Failed to Connect to IP Address: `{}`", x) })?,
        );
        Ok(self)
    }

    /// Connects to an IP Address given.
    ///
    /// # Example
    /// ```no_run
    /// use chalk_protocol::tcp::TCPClient;
    ///
    /// let mut client = TCPClient::new();
    /// client.connect_to("127.0.0.1:8080").unwrap();
    /// ```
    pub fn connect_to<T: ToSocketAddrs>(&mut self, addr: T) -> Result<&mut Self, String> {
        self.connect_to_inner(addr)
    }

    /// Connects to an IP Address given.
    ///
    /// Panics if can't connect to IP Address.
    ///
    /// # Example
    /// ```no_run
    /// use chalk_protocol::tcp::TCPClient;
    ///
    /// let mut client = TCPClient::new();
    /// client.connect_to_unchecked("127.0.0.1:8080");
    /// ```
    pub fn connect_to_unchecked<T: ToSocketAddrs>(&mut self, addr: T) -> &mut Self {
        self.connect_to_inner(addr).unwrap()
    }
}
