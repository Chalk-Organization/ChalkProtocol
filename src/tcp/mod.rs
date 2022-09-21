// TODO: Documentation
use std::sync::{Arc, RwLock};
use tokio::net::{TcpListener, TcpStream};

mod connecting;
mod messaging;

// TODO: Documentation
pub struct TcpClient {
	inner: Arc<InnerTcpClient>,
}

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
