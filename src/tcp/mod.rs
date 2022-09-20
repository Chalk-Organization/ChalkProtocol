use std::{
	net::{TcpListener, TcpStream},
	sync::{Arc, RwLock},
};

pub struct TcpClient {
	inner: Arc<InnerTcpClient>,
}

pub(crate) struct InnerTcpClient {
	streams: RwLock<Vec<RwLock<TcpStream>>>,
	listeners: RwLock<Vec<RwLock<TcpListener>>>,
}

impl TcpClient {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(InnerTcpClient::new()),
		}
	}
}

impl InnerTcpClient {
	pub fn new() -> Self {
		Self {
			streams: RwLock::new(vec![]),
			listeners: RwLock::new(vec![]),
		}
	}
}
