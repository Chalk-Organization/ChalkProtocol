use anyhow::{Ok, Result};
use chalk_protocol::tcp::TcpClient;

#[tokio::test]
pub async fn stream_test() -> Result<()> {
	let client = TcpClient::new();

	assert!(client.write(b"Hello").await.is_err());
	Ok(())
}

#[tokio::test]
pub async fn listener_test() -> Result<()> {
	let client = TcpClient::new();

	assert!(client.read(&mut []).await.is_err());
	Ok(())
}
