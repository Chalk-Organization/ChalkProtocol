// TODO: Documentation
use std::{
	error::Error,
	fmt::Display,
	sync::{Arc, RwLock},
};
use tokio::net::{TcpListener, TcpStream};

mod connecting;
mod messaging;

#[derive(Debug)]
pub enum TcpClientError {
	FailedToWriteStreams,
	FailedToWriteListeners,
	UnboundStream,
	UnboundListener,
}

impl Display for TcpClientError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Error for TcpClientError {}

// TODO: Documentation
pub struct TcpClient {
	inner: Arc<InnerTcpClient>,
}

// TODO: Documentation
pub(crate) struct InnerTcpClient {
	streams: RwLock<Option<TcpStream>>,
	listeners: RwLock<Option<TcpListener>>,
}

impl TcpClient {
	// TODO: Documentation
	pub fn new() -> Self {
		Self {
			inner: Arc::new(InnerTcpClient::new()),
		}
	}
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
