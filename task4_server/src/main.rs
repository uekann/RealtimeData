mod data_holder;
mod server;
mod stock;
mod window;

use anyhow::Result;
#[allow(unused_imports)]
use chrono::Duration as ChronoDuration;
use data_holder::DataHolder;
use std::env;
// use std::io::{BufReader, Read};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use stock::record::Record;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;

#[allow(unused_imports)]
use window::{count_window::CountWindow, time_window::TimeWindow};

use crate::data_holder::StockInfo;
use crate::stock::record::StockKind;

#[tokio::main]
async fn main() -> Result<()> {
    let ip_string = env::var("STREAM_SERVER_IP")?;
    let port_number = env::var("STREAM_SERVER_PORT")?.parse::<u16>()?;

    let server_address = SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number);

    let stream = TcpStream::connect(&server_address).await?;
    // let (reader, _) = stream.into_split();
    // let mut stream_reader = BufReader::new(reader);

    // let window = CountWindow::new(2, 10);
    let window = TimeWindow::new(ChronoDuration::seconds(2), ChronoDuration::seconds(5));
    let mut data_holder = DataHolder::new(window);

    let mut buffer = Vec::new();
    loop {
        let mut tmp_buffer = vec![0; 1024];

        stream.readable().await?;

        match stream.try_read(&mut tmp_buffer) {
            Ok(0) => break,
            Ok(n) => tmp_buffer.truncate(n),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }

        buffer.extend(tmp_buffer.iter().filter(|&&x| x != 0));

        let buffer_string = String::from_utf8_lossy(&buffer);
        let last_crlf = match buffer_string.rfind("\r\n") {
            Some(i) => i,
            None => continue,
        };
        let new_records = buffer_string[..last_crlf]
            .split("\r\n")
            .map(|s| s.into())
            .collect::<Vec<Record>>();

        data_holder.add_records(new_records).await;
        data_holder.update().await;

        if data_holder.is_updated().await {
            println!("----- Current window contents -----");
            let records = data_holder.get_records().await;
            for (timestamp, record) in records {
                println!(
                    "[{}] {}",
                    timestamp.format("%H:%M:%S%.3f"),
                    record.to_string()
                );
            }
            println!("----- Result in the window -----");
            let info_data = data_holder.get_info().await?;
            let mut info = info_data.iter().collect::<Vec<(&StockKind, &StockInfo)>>();
            info.sort_by_key(|(k, _)| **k);
            for (stock_kind, stock_info) in info {
                println!(
                    "Stock name: {}, {}",
                    stock_kind.to_string(),
                    stock_info.to_string()
                );
            }
            println!("===== End of the window =====\n");
        }

        buffer = buffer[last_crlf + 2..].to_vec();
    }

    Ok(())
}
