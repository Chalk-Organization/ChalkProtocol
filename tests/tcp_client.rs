use chalk_protocol::tcp::TCPClient;

fn main() {
    let mut client = TCPClient::new();
    client.connect_to("127.0.0.1:8080").unwrap();
}
