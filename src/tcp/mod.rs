// TODO: Documentation
use std::sync::{Arc, RwLock};
use tokio::net::{TcpListener, TcpStream};

mod connecting;

// TODO: Documentation
pub struct TcpClient {
	inner: Arc<InnerTcpClient>,
}

pub(crate) struct InnerTcpClient {
	streams: RwLock<Vec<RwLock<TcpStream>>>,
	listeners: RwLock<Vec<RwLock<TcpListener>>>,
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
			streams: RwLock::new(vec![]),
			listeners: RwLock::new(vec![]),
		}
	}
}
