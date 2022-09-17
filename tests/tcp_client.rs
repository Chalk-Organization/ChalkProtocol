use chalk_protocol::tcp::TCPClient;

fn main() {
    let mut client = TCPClient::new();
    let data = &mut [0; 3];
    client
        .connect_to("127.0.0.1:8080")
        .unwrap()
        .write(0, &[0, 1, 2])
        .unwrap()
        .read(0, data)
        .unwrap();
    assert_eq!(data, &mut [0, 1, 2]);
}
