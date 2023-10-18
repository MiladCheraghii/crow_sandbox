use std::io::{self, Write};
use std::net::TcpStream;

pub fn send_command(guest_ip: String) -> io::Result<()> {
    let ipPort = format!("{}:7878", guest_ip);
    let mut stream = TcpStream::connect(ipPort)?;
    stream.write_all(b"Hello message to new established connection")?;
    stream.flush()
}