use chrono::NaiveTime;
use std::string::ToString;

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
}

impl ToString for StockKind {
    fn to_string(&self) -> String {
        match self {
            StockKind::A => "A",
            StockKind::B => "B",
            StockKind::C => "C",
            StockKind::D => "D",
            StockKind::E => "E",
            StockKind::F => "F",
            StockKind::G => "G",
            StockKind::H => "H",
            StockKind::I => "I",
            StockKind::J => "J",
            StockKind::K => "K",
            StockKind::L => "L",
            StockKind::M => "M",
            StockKind::N => "N",
            StockKind::O => "O",
            StockKind::P => "P",
            StockKind::Q => "Q",
            StockKind::R => "R",
            StockKind::S => "S",
            StockKind::T => "T",
            StockKind::U => "U",
            StockKind::V => "V",
            StockKind::W => "W",
            StockKind::X => "X",
            StockKind::Y => "Y",
            StockKind::Z => "Z",
        }
        .to_string()
    }
}

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
        stock: char,
        open: f64,
        low: f64,
        high: f64,
        close: f64,
        timestamp: NaiveTime,
    ) -> Record {
        Record {
            stock: StockKind::from_char(stock),
            open,
            low,
            high,
            close,
            timestamp,
        }
    }

    pub fn to_column(&self) -> String {
        format!(
            "stock{},{:.2},{:.2},{:.2},{:.2},{}",
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
        let stock = parts.next().unwrap().chars().last().unwrap();
        let open = parts.next().unwrap().parse().unwrap();
        let low = parts.next().unwrap().parse().unwrap();
        let high = parts.next().unwrap().parse().unwrap();
        let close = parts.next().unwrap().parse().unwrap();
        let timestamp = NaiveTime::parse_from_str(parts.next().unwrap(), "%H:%M:%S%.3f").unwrap();
        Record::new(stock, open, low, high, close, timestamp)
    }
}
