// TODO: Documentation
use std::{
	net::{TcpListener, TcpStream},
	sync::{Arc, RwLock},
};

mod connecting;

// TODO: Documentation
pub struct TcpClient<T> {
	inner: Arc<InnerTcpClient<T>>,
}

pub(crate) struct InnerTcpClient<T> {
	streams: RwLock<Vec<RwLock<TcpStream>>>,
	listeners: RwLock<Vec<RwLock<TcpListener>>>,
	futures: RwLock<Vec<T>>,
}

impl<T> TcpClient<T> {
	// TODO: Documentation
	pub fn new() -> Self {
		Self {
			inner: Arc::new(InnerTcpClient::new()),
		}
	}
}

impl<T> InnerTcpClient<T> {
	// TODO: Documentation
	pub fn new() -> Self {
		Self {
			streams: RwLock::new(vec![]),
			listeners: RwLock::new(vec![]),
			futures: RwLock::new(vec![]),
		}
	}
}
