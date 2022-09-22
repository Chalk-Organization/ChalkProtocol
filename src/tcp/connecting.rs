// TODO: Add Documentation

use std::sync::Arc;

use anyhow::{anyhow, Result};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use super::{InnerTcpClient, TcpClient};

// SECTION: TcpClient

impl TcpClient {
	// TODO: Add Documentation.
	pub async fn connect_to<T: ToSocketAddrs>(&self, address: T) -> Result<&Self> {
		self.inner.clone().connect_to(address).await?;
		Ok(self)
	}

	// TODO: Add Documentation.
	pub async fn bind_to<T: ToSocketAddrs>(&self, address: T) -> Result<&Self> {
		self.inner.clone().bind_to(address).await?;
		Ok(self)
	}
}

// !SECTION
// SECTION: InnerTcpClient

impl InnerTcpClient {
	// TODO: Add Documentation.
	pub async fn connect_to<T: ToSocketAddrs>(self: Arc<Self>, address: T) -> Result<Arc<Self>> {
		let _ = self
			.streams
			.write()
			.map_err(|x| anyhow!("{x}"))?
			.insert(TcpStream::connect(address).await?);
		Ok(self)
	}

	// TODO: Add Documentation.
	pub async fn bind_to<T: ToSocketAddrs>(self: Arc<Self>, address: T) -> Result<Arc<Self>> {
		let _ = self
			.listeners
			.write()
			.map_err(|x| anyhow!("{x}"))?
			.insert(TcpListener::bind(address).await?);
		Ok(self)
	}
}

// !SECTION
