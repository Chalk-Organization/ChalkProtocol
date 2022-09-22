use anyhow::{Ok, Result};
use chalk_protocol::tcp::TcpClient;

#[tokio::test]
pub async fn tcp_test() -> Result<()> {
	let client = TcpClient::new();
	client.bind_to("127.0.0.1:8080").await?;
	client.connect_to("127.0.0.1:8080").await?;

	client.write(&[1; 4]).await?;
	let mut data = [0; 4];
	client.read(&mut data).await?;

	Ok(())
}
