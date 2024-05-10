mod data_holder;
mod stock;

use anyhow::Result;
use data_holder::DataHolder;
use std::env;
use std::io::{BufReader, Read};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use stock::record::Record;
use stock::{count_window::CountWindow, time_window::TimeWindow};

fn main() -> Result<()> {
    let ip_string = env::var("SERVER_IP")?;
    let port_number = env::var("SERVER_PORT")?.parse::<u16>()?;

    let server_address = SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number);

    let stream = TcpStream::connect_timeout(&server_address, Duration::from_secs(5))?;
    let mut stream_reader = BufReader::new(&stream);

    let window = CountWindow::new(100);
    let mut data_holder = DataHolder::new(Box::new(window));

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
            .map(|s| s.into())
            .collect::<Vec<Record>>();

        data_holder.add_records(new_records);
        data_holder.update();

        let info = data_holder.get_info()?;

        println!("New data received");
        for (stock_kind, stock_info) in info {
            println!("{}: {}", stock_kind.to_string(), stock_info.to_string());
        }

        buffer = buffer[last_crlf + 2..].to_vec();
    }

    Ok(())
}
