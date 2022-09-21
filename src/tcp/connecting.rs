// TODO: Add Documentation

use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use super::{InnerTcpClient, TcpClient};

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

impl InnerTcpClient {
	// TODO: Add Documentation.
	pub async fn connect_to<T: ToSocketAddrs>(self: Arc<Self>, address: T) -> Result<Arc<Self>> {
		self.streams
			.write()
			.map_err(|x| anyhow!("{x}"))?
			.push(RwLock::new(TcpStream::connect(address).await?));
		Ok(self)
	}

	// TODO: Add Documentation.
	pub async fn bind_to<T: ToSocketAddrs>(self: Arc<Self>, address: T) -> Result<Arc<Self>> {
		self.listeners
			.write()
			.map_err(|x| anyhow!("{x}"))?
			.push(RwLock::new(TcpListener::bind(address).await?));
		Ok(self)
	}
}
