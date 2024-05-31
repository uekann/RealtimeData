mod data_holder;
mod stock;

use anyhow::Result;
#[allow(unused_imports)]
use chrono::Duration as ChronoDuration;
use chrono::NaiveTime;
use data_holder::DataHolder;
use serde::ser::SerializeSeq;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::io::{BufReader, Read};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use websocket::ws::message;
// use std::thread;
use std::time::Duration;
use stock::record::Record;
use websocket::r#async::server;
use websocket::sync::{client, Server};
use websocket::OwnedMessage;

#[allow(unused_imports)]
use stock::{count_window::CountWindow, time_window::TimeWindow};

use crate::data_holder::StockInfo;
use crate::stock::record::StockKind;

fn main() -> Result<()> {
    let web_server_ip = env::var("WEB_SERVER_IP")?;
    // let web_server_port = env::var("WEB_SERVER_PORT")?.parse::<u16>()?;
    let web_server_port = 3030;

    let server = Server::bind(SocketAddr::new(
        IpAddr::V4(web_server_ip.parse()?),
        web_server_port,
    ))?;

    println!("Listening for connections on port {}", web_server_port);

    for request in server.filter_map(Result::ok) {
        println!("Got a request!");
        if let Ok(mut client) = request.accept() {
            println!("Connection from {}", client.peer_addr()?);

            let stream_server_ip = env::var("STREAM_SERVER_IP")?;
            let stream_server_port = env::var("STREAM_SERVER_PORT")?.parse::<u16>()?;

            let server_address =
                SocketAddr::new(IpAddr::V4(stream_server_ip.parse()?), stream_server_port);

            let stream = TcpStream::connect_timeout(&server_address, Duration::from_secs(10))?;
            let mut stream_reader = BufReader::new(&stream);

            let window = CountWindow::new(1, 20);
            // let window = TimeWindow::new(ChronoDuration::seconds(2), ChronoDuration::seconds(5));
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
                    let records_json = serde_json::to_string(&records)?;
                    println!("{}\n", records_json);
                    // let message = OwnedMessage::Text(records_json);

                    println!("----- Result in the window -----");
                    let info_json = serde_json::to_string(&data_holder.get_info())?;
                    println!("{}\n", info_json);
                    // let message = OwnedMessage::Text(info_json);
                    println!("===== End of the window =====\n");

                    let message = OwnedMessage::Text(format!(
                        r#"{{
                            "records": {},
                            "info": {}
                        }}"#,
                        records_json, info_json,
                    ));
                    client.send_message(&message)?;
                }

                buffer = buffer[last_crlf + 2..].to_vec();
            }
        }
    }

    Ok(())
}
