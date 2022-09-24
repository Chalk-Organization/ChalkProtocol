//! An async wrapper for `TcpStream` and `TcpListener`
//!
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
use std::{
	error::Error,
	fmt::Display,
	sync::{Arc, RwLock},
};
use tokio::net::{TcpListener, TcpStream};

mod connecting;
mod messaging;

// SECTION: TcpClientError

/// Errors for TcpClient
#[derive(Debug)]
pub enum TcpClientError {
	FailedToWriteStreams,
	FailedToWriteListeners,
	UnboundStream,
	UnboundListener,
}

impl Display for TcpClientError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			TcpClientError::FailedToWriteStreams => "Failed to write to Streams",
			TcpClientError::FailedToWriteListeners => "Failed to write to Listeners",
			TcpClientError::UnboundStream => {
				"Stream is not bound (`client.connect_to(\"127.0.0.1:36918\")`)"
			}
			TcpClientError::UnboundListener => {
				"Listener is not bound (`client.bind_to(\"127.0.0.1:36918\")`)"
			}
		})
	}
}

impl Error for TcpClientError {}

// !SECTION
// SECTION: TcpClient

/// Async Tcp Client
pub struct TcpClient {
	inner: Arc<InnerTcpClient>,
}
impl TcpClient {
	/// Creates a new `TcpClient`
	///
	/// # Example
	/// ```
	/// use chalk_protocol::tcp::TcpClient;
	///
	/// let client = TcpClient::new();
	/// ```
	pub fn new() -> Self {
		Self {
			inner: Arc::new(InnerTcpClient::new()),
		}
	}
}

// !SECTION
// SECTION: InnerTcpClient

// Inner Tcp Client to abstract it away from the front end
pub(crate) struct InnerTcpClient {
	streams: RwLock<Option<TcpStream>>,
	listeners: RwLock<Option<TcpListener>>,
}

impl InnerTcpClient {
	/// Creates a new InnerTcpClient
	pub fn new() -> Self {
		Self {
			streams: RwLock::new(None),
			listeners: RwLock::new(None),
		}
	}
}

// !SECTION
