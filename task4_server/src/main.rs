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
use tokio::task::spawn;

#[allow(unused_imports)]
use window::{count_window::CountWindow, time_window::TimeWindow};

use crate::data_holder::StockInfo;
use crate::stock::record::StockKind;

#[tokio::main]
async fn main() -> Result<()> {
    // let ip_string = env::var("STREAM_SERVER_IP")?;
    // let port_number = env::var("STREAM_SERVER_PORT")?.parse::<u16>()?;

    // let server_address = SocketAddr::new(IpAddr::V4(ip_string.parse()?), port_number);

    // let stream = TcpStream::connect(&server_address).await?;
    // // let (reader, _) = stream.into_split();
    // // let mut stream_reader = BufReader::new(reader);

    // // let window = CountWindow::new(2, 10);
    // let window = TimeWindow::new(ChronoDuration::seconds(2), ChronoDuration::seconds(5));
    // let mut data_holder = DataHolder::new(window);

    // let mut server = server::Server::new(data_holder);

    // let recieving_data = spawn(async move {
    //     server.recieving_data(stream).await;
    // });

    // let sending_data = spawn(async move {
    //     server.sending_data().await;
    // });

    // recieving_data.await?;

    Ok(())
}
