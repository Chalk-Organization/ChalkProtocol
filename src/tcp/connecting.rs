// TODO: Add Documentation

use std::{net::SocketAddr, sync::Arc};

use tokio::net::ToSocketAddrs;

use super::{InnerTcpClient, TcpClient};

impl<T> TcpClient<T> {
	// TODO: Add Documentation and Fill in the body.
	pub fn connect_to<A: ToSocketAddrs>(&self, address: A) -> &Self {
		todo!()
	}
}

impl<T> InnerTcpClient<T> {
	// TODO: Add Documentation and Fill in the body.
	pub fn connect_to(self: Arc<Self>, address: SocketAddr) -> Arc<Self> {
		todo!()
	}
}
