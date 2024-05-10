mod stock;

use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::io::{BufReader, Read};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use stock::record::{Record, StockKind};

struct StockInfo {
    min: f64,
    max: f64,
    avg: f64,
    sd: f64,
}

fn classify_records(records: Vec<Record>) -> HashMap<StockKind, Vec<Record>> {
    let mut classified_records: HashMap<StockKind, Vec<Record>> = HashMap::new();
    for record in records {
        let key = record.stock.clone();
        let value = record;
        classified_records
            .entry(key)
            .or_insert(Vec::new())
            .push(value);
    }
    classified_records
}

fn get_info_of_close_value(records: Vec<Record>) -> StockInfo {
    let mut close_values: Vec<f64> = records.iter().map(|r| r.close).collect();
    close_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = *close_values.first().unwrap();
    let max = *close_values.last().unwrap();
    let sum: f64 = close_values.iter().sum();
    let avg = sum / close_values.len() as f64;
    let sd = close_values
        .iter()
        .map(|v| (v - avg).powi(2))
        .sum::<f64>()
        .sqrt()
        / close_values.len() as f64;
    StockInfo { min, max, avg, sd }
}

fn main() -> Result<()> {
    let ip_string = env::var("SERVER_IP")?;
    let port_number = env::var("SERVER_PORT")?.parse::<u16>()?;

    let server_address = SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number);

    let stream = TcpStream::connect_timeout(&server_address, Duration::from_secs(5))?;
    let mut stream_reader = BufReader::new(&stream);

    let mut records: Vec<Record> = Vec::new();

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

        println!("Received {} records", new_records.len());
        new_records
            .iter()
            .for_each(|r| println!("{}", r.to_column()));

        records.extend(new_records);

        buffer = buffer[last_crlf + 2..].to_vec();
    }

    println!("Received {} records in total", records.len());
    Ok(())
}
