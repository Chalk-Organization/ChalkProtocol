// TODO: Add Documentation

use std::sync::Arc;

use anyhow::{Ok, Result};

use super::{InnerTcpClient, TcpClient, TcpClientError};

// SECTION: TcpClient

impl TcpClient {
	// TODO: Add Documentation
	pub async fn read(&self, data: &mut [u8]) -> Result<&Self> {
		self.inner.clone().read(data).await?;
		Ok(self)
	}
		Ok(self)
	}
}

// !SECTION
// SECTION: InnerTcpClient

impl InnerTcpClient {
	// TODO: Add Documentation
	pub async fn read(self: Arc<Self>, data: &mut [u8]) -> Result<Arc<Self>> {
		self.listeners
			.write()
			.map_err(|_| TcpClientError::FailedToWriteListeners)?
			.as_mut()
			.ok_or(TcpClientError::UnboundListener)?
			.accept()
			.await?
			.0
			.try_read(data)?;
		Ok(self)
	}
}

// !SECTION
