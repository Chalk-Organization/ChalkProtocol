// TODO: Add Documentation

use std::sync::Arc;

use anyhow::{Ok, Result};

use super::{InnerTcpClient, TcpClient, TcpClientError};

impl TcpClient {
	// TODO: Add Documentation
	pub async fn read(&self, read: &mut [u8]) -> Result<&Self> {
		self.inner.clone().read(read).await?;
		Ok(self)
	}
}

impl InnerTcpClient {
	// TODO: Add Documentation
	pub async fn read(self: Arc<Self>, read: &mut [u8]) -> Result<Arc<Self>> {
		self.listeners
			.write()
			.map_err(|_| TcpClientError::FailedToWriteListeners)?
			.as_mut()
			.ok_or(TcpClientError::UnboundListener)?
			.accept()
			.await?
			.0
			.try_read(read)?;
		Ok(self)
	}
}
