use std::net::SocketAddrV4;

pub struct UDPClient {
    address: SocketAddrV4,
}

impl UDPClient {
    pub fn new<T: Into<SocketAddrV4>>(ip: T) -> Self {
        Self { address: ip.into() }
    }
}
