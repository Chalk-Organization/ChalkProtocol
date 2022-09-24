//! Connecting with a `TcpStream`
//!
//! # Example
//! ```no_run
//! # use anyhow::{Ok, Result};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! use chalk_protocol::tcp::TcpClient;
//!
//! let client = TcpClient::new();
//! client.bind_to("128.0.0.1:36918").await?;
//! client.connect_to("128.0.0.1:36918").await?;
//! # Ok(())
//! # }
//! ```
use std::sync::Arc;

use anyhow::{anyhow, Result};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use super::{InnerTcpClient, TcpClient};

// SECTION: TcpClient

impl TcpClient {
	/// Creates a `TcpStream`.
	///
	/// # Example
	/// ```
	/// # use anyhow::{Ok, Result};
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// use chalk_protocol::tcp::TcpClient;
	///
	/// let client = TcpClient::new();
	/// client.bind_to("127.0.0.1:36918").await?;
	/// client.connect_to("127.0.0.1:36918").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn connect_to<T: ToSocketAddrs>(&self, address: T) -> Result<&Self> {
		self.inner.clone().connect_to(address).await?;
		Ok(self)
	}

	/// Creates a `TcpListener`.
	///
	/// # Example
	/// ```
	/// # use anyhow::{Ok, Result};
	/// # #[tokio::main]
	/// # async fn main() -> Result<()> {
	/// use chalk_protocol::tcp::TcpClient;
	///
	/// let client = TcpClient::new();
	/// client.bind_to("127.0.0.1:36918").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn bind_to<T: ToSocketAddrs>(&self, address: T) -> Result<&Self> {
		self.inner.clone().bind_to(address).await?;
		Ok(self)
	}
}

// !SECTION
// SECTION: InnerTcpClient

impl InnerTcpClient {
	/// Writes to the `stream` and then creates a `TcpStream`
	pub async fn connect_to<T: ToSocketAddrs>(self: Arc<Self>, address: T) -> Result<Arc<Self>> {
		let _ = self
			.streams
			.write()
			.map_err(|x| anyhow!("{x}"))?
			.insert(TcpStream::connect(address).await?);
		Ok(self)
	}

	/// Writes to the `listeners` and then creates a `TcpListener`
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
