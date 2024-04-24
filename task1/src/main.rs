use chrono::NaiveTime;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{fs::File, io::Write};

const STOCKS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const STOCK_PRICE_AVG: f64 = 38429.96;
const STOCK_PRICE_STD_DEV: f64 = STOCK_PRICE_AVG * 0.3;

const STOCKS_COUNT: usize = 10000;

#[derive(Debug)]
struct Stock {
    name: char,
    open: f64,
    low: f64,
    high: f64,
    close: f64,
    timestamp: NaiveTime,
}

impl Stock {
    fn new(name: char, open: f64, low: f64, high: f64, close: f64, timestamp: NaiveTime) -> Stock {
        Stock {
            name,
            open,
            low,
            high,
            close,
            timestamp,
        }
    }

    fn make_new_stock() -> Stock {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(STOCK_PRICE_AVG, STOCK_PRICE_STD_DEV).unwrap();
        let name = STOCKS[rng.gen_range(0..26)];
        let (low, high) = loop {
            let s1 = normal.sample(&mut rng);
            let s2 = normal.sample(&mut rng);
            if s1 > 0.0 && s2 > 0.0 {
                if s1 < s2 {
                    break (s1, s2);
                } else {
                    break (s2, s1);
                }
            }
        };
        let open = rng.gen_range(low..high);
        let close = rng.gen_range(low..high);
        let timestamp = NaiveTime::from_hms_milli_opt(
            rng.gen_range(0..24),
            rng.gen_range(0..60),
            rng.gen_range(0..60),
            rng.gen_range(0..1000),
        )
        .unwrap();
        Stock::new(name, open, low, high, close, timestamp)
    }

    fn to_column(&self) -> String {
        format!(
            "stock{},{:.2},{:.2},{:.2},{:.2},{}",
            self.name,
            self.open,
            self.low,
            self.high,
            self.close,
            self.timestamp.format("%H:%M:%S%.3f").to_string()
        )
    }
}

fn main() {
    let mut stocks: Vec<Stock> = Vec::new();
    for _ in 0..STOCKS_COUNT {
        let stock = Stock::make_new_stock();
        stocks.push(stock);
    }
    stocks.sort_by(|s1, s2| s1.timestamp.cmp(&s2.timestamp));

    let mut file = File::create("./task1/data/stock_data.txt").unwrap();
    file.write(b"stock,open,low,high,close,timestamp\n")
        .unwrap();
    file.write(
        stocks
            .iter()
            .map(|stock| stock.to_column())
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
}
