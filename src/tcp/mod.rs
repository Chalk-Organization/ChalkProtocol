// TODO: Documentation
use std::{
	error::Error,
	fmt::Display,
	sync::{Arc, RwLock},
};
use tokio::net::{TcpListener, TcpStream};

mod connecting;
mod messaging;

// SECTION: TcpClientError

// TODO: Add Documentation
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

// TODO: Documentation
pub struct TcpClient {
	inner: Arc<InnerTcpClient>,
}
impl TcpClient {
	// TODO: Documentation
	pub fn new() -> Self {
		Self {
			inner: Arc::new(InnerTcpClient::new()),
		}
	}
}

// !SECTION
// SECTION: InnerTcpClient

// TODO: Documentation
pub(crate) struct InnerTcpClient {
	streams: RwLock<Option<TcpStream>>,
	listeners: RwLock<Option<TcpListener>>,
}

impl InnerTcpClient {
	// TODO: Documentation
	pub fn new() -> Self {
		Self {
			streams: RwLock::new(None),
			listeners: RwLock::new(None),
		}
	}
}

// !SECTION
