use super::record::StockKind;
use crate::stock::record::Record;
use std::collections::HashMap;

pub trait Window {
    fn add_record(&mut self, record: Record);
    fn add_records(&mut self, records: Vec<Record>);
    fn is_updated(&self) -> bool;
    fn update(&mut self);
    fn get_records(&mut self) -> Vec<Record>;
    fn get_classify_records(&mut self) -> HashMap<StockKind, Vec<Record>> {
        let mut classified_records: HashMap<StockKind, Vec<Record>> = HashMap::new();
        for record in self.get_records() {
            let key = record.stock.clone();
            let value = record;
            classified_records
                .entry(key)
                .or_insert(Vec::new())
                .push(value);
        }
        classified_records
    }
}
