use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

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
    /// ```no_run
    /// use chalk_protocol::tcp::TCPClient;
    ///
    /// let mut client = TCPClient::new();
    /// client.connect_to("127.0.0.1:8080").unwrap();
    /// ```
    pub fn connect_to<T: ToSocketAddrs>(&mut self, addr: T) -> Result<&mut Self, String> {
        self.streams.push(
            TcpStream::connect(addr)
                // Checks if there is an error an if so then map the error to a custom error
                .map_err(|x| -> String { format!("Failed to Connect to IP Address: `{}`", x) })?,
        );
        Ok(self)
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
        self.connect_to(addr).unwrap()
    }

    /// Write something to a certain index in the streams vec.
    ///
    /// # Example
    /// ```no_run
    /// use chalk_protocol::tcp::TCPClient;
    ///
    /// let mut client = TCPClient::new();
    /// client.connect_to("127.0.0.1:8080").unwrap().write(0, &[0, 1, 2]).unwrap();
    /// ```
    pub fn write(&mut self, idx: usize, data: &[u8]) -> Result<&mut Self, String> {
        match self.streams.get_mut(idx) {
            Some(e) => Ok(e),
            None => Err(String::from("Failed to get index")),
        }?
        .write(data)
        .map_err(|x| format!("Failed to write to IP Address `{}`", x))?;
        Ok(self)
    }

    /// Write something to a certain index in the streams vec.
    ///
    /// Panics if index out of bounds, or fails to write to stream.
    ///
    /// # Example
    /// ```no_run
    /// use chalk_protocol::tcp::TCPClient;
    ///
    /// let mut client = TCPClient::new();
    /// client.connect_to("127.0.0.1:8080").unwrap().write(0, &[0, 1, 2]);
    /// ```
    pub fn write_unchecked(&mut self, idx: usize, data: &[u8]) -> &mut Self {
        self.write(idx, data).unwrap()
    }

    /// Read something from a certain stream index
    ///
    /// # Example
    /// ```no_run
    /// use chalk_protocol::tcp::TCPClient;
    ///
    /// let mut client = TCPClient::new();
    /// let data = &mut [];
    /// client.connect_to("127.0.0.1:8080").unwrap().read(0, &data);
    /// ```
    pub fn read(&mut self, idx: usize, data: &mut [u8]) -> Result<&mut Self, String> {
        match self.streams.get_mut(idx) {
            Some(e) => Ok(e),
            None => Err(String::from("Failed to get index")),
        }?
        .read(data)
        .map_err(|x| format!("Failed to read IP Address `{}`", x))?;
        Ok(self)
    }

    /// Read something from a certain stream index
    ///
    /// Panics if can't read or index out of bounds.
    ///
    /// # Example
    /// ```no_run
    /// use chalk_protocol::tcp::TCPClient;
    ///
    /// let mut client = TCPClient::new();
    /// let data = &mut [];
    /// client.connect_to("127.0.0.1:8080").unwrap().read_unchecked(0, &data);
    /// ```
    pub fn read_unchecked(&mut self, idx: usize, data: &mut [u8]) -> &mut Self {
        self.read(idx, data).unwrap()
    }
}
