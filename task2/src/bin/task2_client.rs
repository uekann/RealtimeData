use anyhow::Result;
use chrono::Local;
use std::env;
use std::io::{BufReader, Read};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

fn main() -> Result<()> {
    let ip_string = env::var("STREAM_SERVER_IP")?;
    let port_number = env::var("STREAM_SERVER_PORT")?.parse::<u16>()?;

    let server_address = SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number);

    let stream = TcpStream::connect_timeout(&server_address, Duration::from_secs(5))?;
    let mut stream_reader = BufReader::new(&stream);

    let mut records = Vec::new();

    let mut buffer = Vec::new();
    loop {
        let mut tmp_buffer = vec![0; 1024];
        if stream_reader.read(&mut tmp_buffer)? == 0 {
            break;
        }

        buffer.extend_from_slice(&tmp_buffer);

        let buffer_string = String::from_utf8_lossy(&buffer);
        let last_crlf = match buffer_string.rfind("\r\n") {
            Some(i) => i,
            None => continue,
        };
        let new_records = buffer_string[..last_crlf]
            .split("\r\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        println!("Received {} records", new_records.len());
        new_records.iter().for_each(|r| {
            println!(
                "{}, {}",
                r,
                Local::now().naive_local().time().format("%H:%M:%S%.3f")
            )
        });
        // new_records.last().map(|r| println!("Last record : {}", r));
        records.extend(new_records);

        buffer = buffer[last_crlf + 2..].to_vec();
    }

    println!("Received {} records in total", records.len());
    Ok(())
}
