use std::io::{self, Write};
use std::net::TcpStream;

pub fn send_command() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write_all(b"Hello message to new established connection")?;
    stream.flush()
}