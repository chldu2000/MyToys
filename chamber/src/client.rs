use std::net::TcpStream;

pub struct Client {
    username: String,
    stream: TcpStream,
}
impl Client {
    pub fn init(&mut self) -> std::io::Result<()> {
        self.username = String::from("MyUser");
        self.stream = TcpStream::connect("127.0.0.1:9999")?;

        return Ok(());
    }
}
