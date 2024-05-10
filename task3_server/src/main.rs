use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener};

fn main() -> Result<()> {
    // Start listening for incoming connections
    let ip_string = env::var("SERVER_IP")?;
    let port_number = env::var("SERVER_PORT")?.parse::<u16>()?;
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number))
        .expect("Failed to bind to address");

    println!("Listening for connections on port {}", port_number);

    // Accept incoming connections
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("Accepted connection from {:?}", addr);

            // Open file and send data to client
            let file = File::open("./task3_server/data/stock_data_without_timestamp.txt")?;
            let file_reader = BufReader::new(file);

            let mut stream_writer = BufWriter::new(&stream);

            println!("Sending data to client...");

            // Skip the header line
            let mut lines = file_reader.lines();
            lines.next();

            // Send data to client
            for mut line in lines.map(|l| l.expect("Failed to read line")) {
                line.push_str("\r\n");
                stream_writer.write_all(line.as_bytes())?;

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
