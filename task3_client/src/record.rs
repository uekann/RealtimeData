use chrono::NaiveTime;

pub struct Record {
    stock: char,
    open: f64,
    low: f64,
    high: f64,
    close: f64,
    timestamp: NaiveTime,
}

impl Record {
    fn new(
        stock: char,
        open: f64,
        low: f64,
        high: f64,
        close: f64,
        timestamp: NaiveTime,
    ) -> Record {
        Record {
            stock,
            open,
            low,
            high,
            close,
            timestamp,
        }
    }
}

impl From<&str> for Record {
    fn from(s: &str) -> Record {
        let mut parts = s.split(",");
        let stock = parts.next().unwrap().chars().next().unwrap();
        let open = parts.next().unwrap().parse().unwrap();
        let low = parts.next().unwrap().parse().unwrap();
        let high = parts.next().unwrap().parse().unwrap();
        let close = parts.next().unwrap().parse().unwrap();
        let timestamp = NaiveTime::parse_from_str(parts.next().unwrap(), "%H:%M:%S%.3f").unwrap();
        Record::new(stock, open, low, high, close, timestamp)
    }
}
