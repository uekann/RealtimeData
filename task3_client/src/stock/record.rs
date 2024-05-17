use chrono::{Local, NaiveTime};
use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
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
    // pub fn from_str(s: &str) -> StockKind {
    //     match s {
    //         "stockA" => StockKind::A,
    //         "stockB" => StockKind::B,
    //         "stockC" => StockKind::C,
    //         "stockD" => StockKind::D,
    //         "stockE" => StockKind::E,
    //         "stockF" => StockKind::F,
    //         "stockG" => StockKind::G,
    //         "stockH" => StockKind::H,
    //         "stockI" => StockKind::I,
    //         "stockJ" => StockKind::J,
    //         "stockK" => StockKind::K,
    //         "stockL" => StockKind::L,
    //         "stockM" => StockKind::M,
    //         "stockN" => StockKind::N,
    //         "stockO" => StockKind::O,
    //         "stockP" => StockKind::P,
    //         "stockQ" => StockKind::Q,
    //         "stockR" => StockKind::R,
    //         "stockS" => StockKind::S,
    //         "stockT" => StockKind::T,
    //         "stockU" => StockKind::U,
    //         "stockV" => StockKind::V,
    //         "stockW" => StockKind::W,
    //         "stockX" => StockKind::X,
    //         "stockY" => StockKind::Y,
    //         "stockZ" => StockKind::Z,
    //         _ => panic!("Invalid stock kind"),
    //     }
    // }

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
    // fn to_string(&self) -> String {
    //     match self {
    //         StockKind::A => "stockA",
    //         StockKind::B => "stockB",
    //         StockKind::C => "stockC",
    //         StockKind::D => "stockD",
    //         StockKind::E => "stockE",
    //         StockKind::F => "stockF",
    //         StockKind::G => "stockG",
    //         StockKind::H => "stockH",
    //         StockKind::I => "stockI",
    //         StockKind::J => "stockJ",
    //         StockKind::K => "stockK",
    //         StockKind::L => "stockL",
    //         StockKind::M => "stockM",
    //         StockKind::N => "stockN",
    //         StockKind::O => "stockO",
    //         StockKind::P => "stockP",
    //         StockKind::Q => "stockQ",
    //         StockKind::R => "stockR",
    //         StockKind::S => "stockS",
    //         StockKind::T => "stockT",
    //         StockKind::U => "stockU",
    //         StockKind::V => "stockV",
    //         StockKind::W => "stockW",
    //         StockKind::X => "stockX",
    //         StockKind::Y => "stockY",
    //         StockKind::Z => "stockZ",
    //     }
    //     .to_string()
    // }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_kind_from_str() {
        assert_eq!(StockKind::from_str("stockA"), StockKind::A);
        assert_eq!(StockKind::from_str("stockB"), StockKind::B);
        assert_eq!(StockKind::from_str("stockC"), StockKind::C);
        assert_eq!(StockKind::from_str("stockD"), StockKind::D);
        assert_eq!(StockKind::from_str("stockE"), StockKind::E);
        assert_eq!(StockKind::from_str("stockF"), StockKind::F);
        assert_eq!(StockKind::from_str("stockG"), StockKind::G);
        assert_eq!(StockKind::from_str("stockH"), StockKind::H);
        assert_eq!(StockKind::from_str("stockI"), StockKind::I);
        assert_eq!(StockKind::from_str("stockJ"), StockKind::J);
        assert_eq!(StockKind::from_str("stockK"), StockKind::K);
        assert_eq!(StockKind::from_str("stockL"), StockKind::L);
        assert_eq!(StockKind::from_str("stockM"), StockKind::M);
        assert_eq!(StockKind::from_str("stockN"), StockKind::N);
        assert_eq!(StockKind::from_str("stockO"), StockKind::O);
        assert_eq!(StockKind::from_str("stockP"), StockKind::P);
        assert_eq!(StockKind::from_str("stockQ"), StockKind::Q);
        assert_eq!(StockKind::from_str("stockR"), StockKind::R);
        assert_eq!(StockKind::from_str("stockS"), StockKind::S);
        assert_eq!(StockKind::from_str("stockT"), StockKind::T);
        assert_eq!(StockKind::from_str("stockU"), StockKind::U);
        assert_eq!(StockKind::from_str("stockV"), StockKind::V);
        assert_eq!(StockKind::from_str("stockW"), StockKind::W);
        assert_eq!(StockKind::from_str("stockX"), StockKind::X);
        assert_eq!(StockKind::from_str("stockY"), StockKind::Y);
        assert_eq!(StockKind::from_str("stockZ"), StockKind::Z);
    }

    #[test]
    fn test_stock_kind_to_string() {
        assert_eq!(StockKind::A.to_string(), "stockA");
        assert_eq!(StockKind::B.to_string(), "stockB");
        assert_eq!(StockKind::C.to_string(), "stockC");
        assert_eq!(StockKind::D.to_string(), "stockD");
        assert_eq!(StockKind::E.to_string(), "stockE");
        assert_eq!(StockKind::F.to_string(), "stockF");
        assert_eq!(StockKind::G.to_string(), "stockG");
        assert_eq!(StockKind::H.to_string(), "stockH");
        assert_eq!(StockKind::I.to_string(), "stockI");
        assert_eq!(StockKind::J.to_string(), "stockJ");
        assert_eq!(StockKind::K.to_string(), "stockK");
        assert_eq!(StockKind::L.to_string(), "stockL");
        assert_eq!(StockKind::M.to_string(), "stockM");
        assert_eq!(StockKind::N.to_string(), "stockN");
        assert_eq!(StockKind::O.to_string(), "stockO");
        assert_eq!(StockKind::P.to_string(), "stockP");
        assert_eq!(StockKind::Q.to_string(), "stockQ");
        assert_eq!(StockKind::R.to_string(), "stockR");
        assert_eq!(StockKind::S.to_string(), "stockS");
        assert_eq!(StockKind::T.to_string(), "stockT");
        assert_eq!(StockKind::U.to_string(), "stockU");
        assert_eq!(StockKind::V.to_string(), "stockV");
        assert_eq!(StockKind::W.to_string(), "stockW");
        assert_eq!(StockKind::X.to_string(), "stockX");
        assert_eq!(StockKind::Y.to_string(), "stockY");
        assert_eq!(StockKind::Z.to_string(), "stockZ");
    }

    #[test]
    fn test_record_new() {
        let record = Record::new(
            "stockA",
            1.0,
            2.0,
            3.0,
            4.0,
            NaiveTime::from_hms_opt(12, 34, 56).unwrap(),
        );
        assert_eq!(record.stock, StockKind::A);
        assert_eq!(record.open, 1.0);
        assert_eq!(record.low, 2.0);
        assert_eq!(record.high, 3.0);
        assert_eq!(record.close, 4.0);
        assert_eq!(
            record.timestamp,
            NaiveTime::from_hms_opt(12, 34, 56).unwrap()
        );
    }

    #[test]
    fn test_record_to_column() {
        let record = Record::new(
            "stockA",
            1.0,
            2.0,
            3.0,
            4.0,
            NaiveTime::from_hms_opt(12, 34, 56).unwrap(),
        );
        assert_eq!(record.to_string(), "stockA,1.00,2.00,3.00,4.00");
    }

    #[test]
    fn test_record_from_str() {
        let record = Record::from("stockA,1.00,2.00,3.00,4.00");
        assert_eq!(record.stock, StockKind::A);
        assert_eq!(record.open, 1.0);
        assert_eq!(record.low, 2.0);
        assert_eq!(record.high, 3.0);
        assert_eq!(record.close, 4.0);
    }
}
