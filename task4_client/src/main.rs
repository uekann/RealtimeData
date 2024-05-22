mod data_holder;
mod stock;

use anyhow::Result;
#[allow(unused_imports)]
use chrono::Duration as ChronoDuration;
use data_holder::DataHolder;
use std::env;
use std::io::{BufReader, Read};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use stock::record::Record;

#[allow(unused_imports)]
use stock::{count_window::CountWindow, time_window::TimeWindow};

use crate::data_holder::StockInfo;
use crate::stock::record::StockKind;

fn main() -> Result<()> {
    let ip_string = env::var("SERVER_IP")?;
    let port_number = env::var("SERVER_PORT")?.parse::<u16>()?;

    let server_address = SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number);

    let stream = TcpStream::connect_timeout(&server_address, Duration::from_secs(10))?;
    let mut stream_reader = BufReader::new(&stream);

    // let window = CountWindow::new(2, 10);
    let window = TimeWindow::new(
        ChronoDuration::seconds(2),
        ChronoDuration::seconds(5),
    );
    let mut data_holder = DataHolder::new(Box::new(window));

    let mut buffer = Vec::new();
    loop {
        let mut tmp_buffer = vec![0; 1024];
        if stream_reader.read(&mut tmp_buffer)? == 0 {
            break;
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

        data_holder.add_records(new_records);
        data_holder.update();

        if data_holder.is_updated() {
            println!("----- Current window contents -----");
            let records = data_holder.get_records();
            for record in records {
                println!(
                    "[{}] {}",
                    record.timestamp.format("%H:%M:%S%.3f"),
                    record.to_string()
                );
            }
            println!("----- Result in the window -----");
            let info_data = data_holder.get_info()?;
            let mut info = info_data.iter().collect::<Vec<(&StockKind, &StockInfo)>>();
            info.sort_by_key(|(k, _)|  **k );
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
