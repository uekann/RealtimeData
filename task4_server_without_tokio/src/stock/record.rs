use chrono::{Local, NaiveTime};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Deserialize)]
pub enum StockKind {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl StockKind {
    pub fn from_str(s: &str) -> StockKind {
        match s {
            "株A" => StockKind::A,
            "株B" => StockKind::B,
            "株C" => StockKind::C,
            "株D" => StockKind::D,
            "株E" => StockKind::E,
            "株F" => StockKind::F,
            "株G" => StockKind::G,
            "株H" => StockKind::H,
            "株I" => StockKind::I,
            "株J" => StockKind::J,
            "株K" => StockKind::K,
            "株L" => StockKind::L,
            "株M" => StockKind::M,
            "株N" => StockKind::N,
            "株O" => StockKind::O,
            "株P" => StockKind::P,
            "株Q" => StockKind::Q,
            "株R" => StockKind::R,
            "株S" => StockKind::S,
            "株T" => StockKind::T,
            "株U" => StockKind::U,
            "株V" => StockKind::V,
            "株W" => StockKind::W,
            "株X" => StockKind::X,
            "株Y" => StockKind::Y,
            "株Z" => StockKind::Z,
            _ => panic!("Invalid stock kind"),
        }
    }
}

impl ToString for StockKind {
    fn to_string(&self) -> String {
        match self {
            StockKind::A => "株A",
            StockKind::B => "株B",
            StockKind::C => "株C",
            StockKind::D => "株D",
            StockKind::E => "株E",
            StockKind::F => "株F",
            StockKind::G => "株G",
            StockKind::H => "株H",
            StockKind::I => "株I",
            StockKind::J => "株J",
            StockKind::K => "株K",
            StockKind::L => "株L",
            StockKind::M => "株M",
            StockKind::N => "株N",
            StockKind::O => "株O",
            StockKind::P => "株P",
            StockKind::Q => "株Q",
            StockKind::R => "株R",
            StockKind::S => "株S",
            StockKind::T => "株T",
            StockKind::U => "株U",
            StockKind::V => "株V",
            StockKind::W => "株W",
            StockKind::X => "株X",
            StockKind::Y => "株Y",
            StockKind::Z => "株Z",
        }
        .to_string()
    }
}

impl Serialize for StockKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub stock: StockKind,
    pub open: f64,
    pub low: f64,
    pub high: f64,
    pub close: f64,
}

impl Record {
    pub fn new(stock: &str, open: f64, low: f64, high: f64, close: f64) -> Record {
        Record {
            stock: StockKind::from_str(stock),
            open,
            low,
            high,
            close,
        }
    }
}

impl From<&str> for Record {
    fn from(s: &str) -> Record {
        let mut parts = s.split(",");
        let stock = parts.next().unwrap();
        let open = parts.next().unwrap().parse().unwrap();
        let low = parts.next().unwrap().parse().unwrap();
        let high = parts.next().unwrap().parse().unwrap();
        let close = parts.next().unwrap().parse().unwrap();
        Record::new(stock, open, low, high, close)
    }
}

impl ToString for Record {
    fn to_string(&self) -> String {
        format!(
            "{},{:.2},{:.2},{:.2},{:.2}",
            self.stock.to_string(),
            self.open,
            self.low,
            self.high,
            self.close,
        )
    }
}
