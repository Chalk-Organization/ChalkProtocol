// TODO: Add Documentation

use std::{net::SocketAddr, sync::Arc};

use tokio::net::ToSocketAddrs;

use super::{InnerTcpClient, TcpClient};

impl TcpClient {
	// TODO: Add Documentation and Fill in the body.
	pub fn connect_to<T: ToSocketAddrs>(&self, address: T) -> &Self {
		todo!()
	}
}

impl InnerTcpClient {
	// TODO: Add Documentation and Fill in the body.
	pub fn connect_to(self: Arc<Self>, address: SocketAddr) -> Arc<Self> {
		todo!()
	}
}
