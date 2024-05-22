use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener};

fn main() -> Result<()> {
    let ip_string = env::var("STREAM_SERVER_IP")?;
    let port_number = env::var("STREAM_SERVER_PORT")?.parse::<u16>()?;

    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number))
        .expect("Failed to bind to address");

    println!("Listening for connections on port {}", port_number);

    match listener.accept() {
        Ok((stream, addr)) => {
            println!("Accepted connection from {:?}", addr);

            let file = File::open("./task2/data/stock_data_without_timestamp.txt")?;
            let file_reader = BufReader::new(file);

            let mut stream_writer = BufWriter::new(&stream);

            println!("Sending data to client...");
            for mut line in file_reader.lines().map(|l| l.expect("Failed to read line")) {
                line.push_str("\r\n");
                stream_writer.write_all(line.as_bytes())?;
                stream_writer.flush()?;

                // 0.01s delay
                std::thread::sleep(std::time::Duration::from_millis(500));
            }

            stream_writer.flush()?;
            println!("Data sent to client");
            stream.shutdown(Shutdown::Both)?;

            println!("Connection closed");
        }
        Err(e) => {
            eprintln!("Accept error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
