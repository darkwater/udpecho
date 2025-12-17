use std::net::UdpSocket;
use std::path::Path;
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:9999".to_owned());
    let secret_file = env::var("SECRET_FILE").unwrap_or_else(|_| "/etc/hostname".to_owned());
    const MAX_PACKET_SIZE: usize = 256;

    // Read secret once
    let secret = fs::read(Path::new(&secret_file))?;

    // Bind socket
    let socket = UdpSocket::bind(bind_addr)?;
    socket.set_nonblocking(false)?;

    let mut buf = [0u8; MAX_PACKET_SIZE];

    loop {
        let (len, peer) = socket.recv_from(&mut buf)?;

        if buf[..len] == secret {
            // Echo back the secret
            let _ = socket.send_to(&secret, peer);
        }
        // else: silently drop
    }
}
