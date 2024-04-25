use anyhow::Result;
use std::env;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};

fn main() -> Result<()> {
    let ip_string = env::var("SERVER_IP")?;
    let port_number = env::var("SERVER_PORT")?.parse::<u16>()?;

    let server_address = SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number);

    let mut stream = match TcpStream::connect(server_address) {
        Ok(stream) => stream,
        Err(_) => {
            println!("\nConnection Failed \n");
            std::process::exit(1);
        }
    };

    let message = "Hello from client";
    match stream.write_all(message.as_bytes()) {
        Ok(_) => println!("Message sent to server"),
        Err(e) => {
            eprintln!("Error sending message: {}", e);
            std::process::exit(1);
        }
    }

    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received message is \"{}\"", received_message);
        }
        Err(e) => {
            eprintln!("Error reading from server: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
