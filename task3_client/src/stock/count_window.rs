use crate::stock::record::Record;
use crate::stock::window::Window;

pub struct CountWindow {
    window_size: usize,
    records: Vec<Record>,
}

impl CountWindow {
    pub fn new(window_size: usize) -> CountWindow {
        CountWindow {
            window_size,
            records: Vec::new(),
        }
    }
}

impl Window for CountWindow {
    fn add_record(&mut self, record: Record) {
        self.records.push(record);
    }

    fn add_records(&mut self, records: Vec<Record>) {
        self.records.extend(records);
    }

    fn update(&mut self) {
        if self.records.len() > self.window_size {
            self.records = self
                .records
                .split_off(self.records.len() - self.window_size);
        }
    }

    fn get_records(&self) -> Vec<Record> {
        self.records.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stock::record::StockKind;
    use chrono::NaiveDateTime;
    use std::collections::HashMap;

    #[test]
    fn test_count_window() {
        let mut window = CountWindow::new(3);
        let record1 = Record::from("stockA,1.0,1.0,1.0,1.0");
        let record2 = Record::from("stockA,2.0,2.0,2.0,2.0");
        let record3 = Record::from("stockA,3.0,3.0,3.0,3.0");
        let record4 = Record::from("stockA,4.0,4.0,4.0,4.0");
        let record5 = Record::from("stockA,5.0,5.0,5.0,5.0");

        window.add_record(record1);
        window.add_record(record2);
        window.add_record(record3);
        window.add_record(record4);
        window.add_record(record5);

        window.update();

        let records = window.get_records();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].close, 3.0);
        assert_eq!(records[1].close, 4.0);
        assert_eq!(records[2].close, 5.0);

        let mut classified_records = HashMap::new();
        classified_records.insert(StockKind::A, records);
        let classified_records = window.get_classify_records();
        assert_eq!(classified_records.len(), 1);
        assert_eq!(classified_records[&StockKind::A].len(), 3);
        assert_eq!(classified_records[&StockKind::A][0].close, 3.0);
        assert_eq!(classified_records[&StockKind::A][1].close, 4.0);
        assert_eq!(classified_records[&StockKind::A][2].close, 5.0);
    }

    #[test]
    fn test_count_window_with_different_stock() {
        let mut window = CountWindow::new(3);
        let record1 = Record::from("stockA,1.0,1.0,1.0,1.0");
        let record2 = Record::from("stockB,2.0,2.0,2.0,2.0");
        let record3 = Record::from("stockA,3.0,3.0,3.0,3.0");
        let record4 = Record::from("stockB,4.0,4.0,4.0,4.0");
        let record5 = Record::from("stockA,5.0,5.0,5.0,5.0");

        window.add_record(record1);
        window.add_record(record2);
        window.add_record(record3);
        window.add_record(record4);
        window.add_record(record5);

        window.update();

        let records = window.get_records();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].close, 3.0);
        assert_eq!(records[1].close, 4.0);
        assert_eq!(records[2].close, 5.0);

        let mut classified_records = HashMap::new();
        classified_records.insert(StockKind::A, vec![records[0].clone(), records[2].clone()]);
        classified_records.insert(StockKind::B, vec![records[1].clone()]);
        let classified_records = window.get_classify_records();
        assert_eq!(classified_records.len(), 2);
        assert_eq!(classified_records[&StockKind::A].len(), 2);
        assert_eq!(classified_records[&StockKind::A][0].close, 3.0);
        assert_eq!(classified_records[&StockKind::A][1].close, 5.0);
        assert_eq!(classified_records[&StockKind::B].len(), 1);
        assert_eq!(classified_records[&StockKind::B][0].close, 4.0);
    }
}
