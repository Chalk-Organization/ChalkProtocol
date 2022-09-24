//! Write and Reading for the `TcpClient`
//!
//! # Example
//! ```
//! # use anyhow::{Ok, Result};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! use chalk_protocol::tcp::TcpClient;
//!
//! let client = TcpClient::new();
//! client.bind_to("127.0.0.1:36918").await?;
//! client.connect_to("127.0.0.1:36918").await?;
//!
//! client.write(b"hello").await?;
//! let mut data = [0; 5];  
//! client.read(&mut data).await?;
//! # Ok(())
//! # }
//! ```
use std::sync::Arc;

use anyhow::{Ok, Result};

use super::{InnerTcpClient, TcpClient, TcpClientError};

// SECTION: TcpClient

impl TcpClient {
	/// Reads from a Client to get any sent data.
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
	///
	/// client.write(b"hello").await?;
	/// let mut data = [0; 5];  
	/// client.read(&mut data).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn read(&self, data: &mut [u8]) -> Result<&Self> {
		self.inner.clone().read(data).await?;
		Ok(self)
	}

	/// Sends data to a bound listener.
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
	///
	/// client.write(b"hello").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn write(&self, data: &[u8]) -> Result<&Self> {
		self.inner.clone().write(data).await?;
		Ok(self)
	}
}

// !SECTION
// SECTION: InnerTcpClient

impl InnerTcpClient {
	/// Reads from a TcpListener.
	pub async fn read(self: Arc<Self>, data: &mut [u8]) -> Result<Arc<Self>> {
		let stream = self
			.listeners
			.write()
			.map_err(|_| TcpClientError::FailedToWriteListeners)?
			.as_mut()
			.ok_or(TcpClientError::UnboundListener)?
			.accept()
			.await?
			.0;
		stream.readable().await?;
		stream.try_read(data)?;
		Ok(self)
	}

	/// Writes to a TcpStream.
	pub async fn write(&self, data: &[u8]) -> Result<&Self> {
		self.streams
			.write()
			.map_err(|_| TcpClientError::FailedToWriteStreams)?
			.as_mut()
			.ok_or(TcpClientError::UnboundStream)?
			.try_write(data)?;
		Ok(self)
	}
}

// !SECTION
