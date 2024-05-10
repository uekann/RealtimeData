use chrono::{Local, NaiveTime};
use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    pub fn from_char(c: char) -> StockKind {
        match c {
            'A' => StockKind::A,
            'B' => StockKind::B,
            'C' => StockKind::C,
            'D' => StockKind::D,
            'E' => StockKind::E,
            'F' => StockKind::F,
            'G' => StockKind::G,
            'H' => StockKind::H,
            'I' => StockKind::I,
            'J' => StockKind::J,
            'K' => StockKind::K,
            'L' => StockKind::L,
            'M' => StockKind::M,
            'N' => StockKind::N,
            'O' => StockKind::O,
            'P' => StockKind::P,
            'Q' => StockKind::Q,
            'R' => StockKind::R,
            'S' => StockKind::S,
            'T' => StockKind::T,
            'U' => StockKind::U,
            'V' => StockKind::V,
            'W' => StockKind::W,
            'X' => StockKind::X,
            'Y' => StockKind::Y,
            'Z' => StockKind::Z,
            _ => panic!("Invalid stock kind"),
        }
    }

    pub fn from_str(s: &str) -> StockKind {
        match s {
            "stockA" => StockKind::A,
            "stockB" => StockKind::B,
            "stockC" => StockKind::C,
            "stockD" => StockKind::D,
            "stockE" => StockKind::E,
            "stockF" => StockKind::F,
            "stockG" => StockKind::G,
            "stockH" => StockKind::H,
            "stockI" => StockKind::I,
            "stockJ" => StockKind::J,
            "stockK" => StockKind::K,
            "stockL" => StockKind::L,
            "stockM" => StockKind::M,
            "stockN" => StockKind::N,
            "stockO" => StockKind::O,
            "stockP" => StockKind::P,
            "stockQ" => StockKind::Q,
            "stockR" => StockKind::R,
            "stockS" => StockKind::S,
            "stockT" => StockKind::T,
            "stockU" => StockKind::U,
            "stockV" => StockKind::V,
            "stockW" => StockKind::W,
            "stockX" => StockKind::X,
            "stockY" => StockKind::Y,
            "stockZ" => StockKind::Z,
            _ => panic!("Invalid stock kind"),
        }
    }
}

impl ToString for StockKind {
    fn to_string(&self) -> String {
        match self {
            StockKind::A => "stockA",
            StockKind::B => "stockB",
            StockKind::C => "stockC",
            StockKind::D => "stockD",
            StockKind::E => "stockE",
            StockKind::F => "stockF",
            StockKind::G => "stockG",
            StockKind::H => "stockH",
            StockKind::I => "stockI",
            StockKind::J => "stockJ",
            StockKind::K => "stockK",
            StockKind::L => "stockL",
            StockKind::M => "stockM",
            StockKind::N => "stockN",
            StockKind::O => "stockO",
            StockKind::P => "stockP",
            StockKind::Q => "stockQ",
            StockKind::R => "stockR",
            StockKind::S => "stockS",
            StockKind::T => "stockT",
            StockKind::U => "stockU",
            StockKind::V => "stockV",
            StockKind::W => "stockW",
            StockKind::X => "stockX",
            StockKind::Y => "stockY",
            StockKind::Z => "stockZ",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub stock: StockKind,
    pub open: f64,
    pub low: f64,
    pub high: f64,
    pub close: f64,
    pub timestamp: NaiveTime,
}

impl Record {
    pub fn new(
        stock: &str,
        open: f64,
        low: f64,
        high: f64,
        close: f64,
        timestamp: NaiveTime,
    ) -> Record {
        Record {
            stock: StockKind::from_str(stock),
            open,
            low,
            high,
            close,
            timestamp,
        }
    }

    pub fn to_column(&self) -> String {
        format!(
            "{},{:.2},{:.2},{:.2},{:.2},{}",
            self.stock.to_string(),
            self.open,
            self.low,
            self.high,
            self.close,
            self.timestamp.format("%H:%M:%S%.3f").to_string()
        )
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
        let timestamp = Local::now().naive_local().time();
        Record::new(stock, open, low, high, close, timestamp)
    }
}
