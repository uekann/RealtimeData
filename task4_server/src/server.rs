use std::sync::{Arc, Mutex};

use crate::data_holder::{DataHolder, StockInfo};
use crate::stock::record::{Record, StockKind};
use crate::window::window::Window;

// use tokio::sync::Mutex;
use tokio::task::spawn;
use tokio::task::JoinHandle;

#[derive(Clone)]
struct Receiver<W: Window<Record> + Clone + Send + 'static> {
    data_holder: Arc<DataHolder<W>>,
}

#[derive(Clone)]
struct Sender<W: Window<Record> + Clone + Send + 'static> {
    data_holder: Arc<DataHolder<W>>,
}

pub struct Server<W: Window<Record> + Clone + Send + 'static> {
    data_holder: Arc<DataHolder<W>>,
    receiver: Receiver<W>,
    sender: Sender<W>,
    is_recieving: bool,
    // data_holder: DataHolder<W>,
}

impl<W: Window<Record> + Clone + Send + 'static> Receiver<W> {
    pub async fn recieving_data(self, stream: tokio::net::TcpStream) {
        let mut buffer = Vec::new();
        loop {
            let mut tmp_buffer = vec![0; 1024];

            stream.readable().await.unwrap();

            match stream.try_read(&mut tmp_buffer) {
                Ok(0) => break,
                Ok(n) => tmp_buffer.truncate(n),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) => panic!("{}", e),
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

            self.data_holder.add_records(new_records).await;

            buffer = buffer[last_crlf + 2..].to_vec();
        }
    }
}

impl<W: Window<Record> + Clone + Send + 'static> Sender<W> {
    pub async fn sending_data(self) {
        loop {
            self.data_holder.update().await;

            if self.data_holder.is_updated().await {
                println!("----- Current window contents -----");
                let records = self.data_holder.get_records().await;
                for (timestamp, record) in records {
                    println!(
                        "[{}] {}",
                        timestamp.format("%H:%M:%S%.3f"),
                        record.to_string()
                    );
                }
                let info_data = self.data_holder.get_info().await.unwrap();
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
        }
    }
}

impl<W: Window<Record> + Clone + Send + 'static> Server<W> {
    pub fn new(data_holder: DataHolder<W>) -> Server<W> {
        let data_holder = Arc::new(data_holder);
        Server {
            data_holder: Arc::clone(&data_holder),
            receiver: Receiver {
                data_holder: Arc::clone(&data_holder),
            },
            sender: Sender {
                data_holder: Arc::clone(&data_holder),
            },
            is_recieving: false,
        }
    }

    pub async fn start_sending(self) -> JoinHandle<()> {
        let sender = self.sender.clone();
        spawn(async move {
            sender.sending_data().await;
        })
    }

    // pub async fn start_recieving(self, stream: tokio::net::TcpStream) -> JoinHandle<()> {
    //     self.is_recieving = true;
    //     let receiver = self.receiver.clone();
    //     let handle = spawn(async move {
    //         receiver.recieving_data(stream).await;
    //     });
    //     handle
    // }
}
