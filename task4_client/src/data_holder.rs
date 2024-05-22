use crate::stock::record::{Record, StockKind};
use crate::stock::window::Window;
use anyhow::Result;
use std::collections::HashMap;

pub struct StockInfo {
    min: f64,
    max: f64,
    avg: f64,
    std: f64,
}

impl ToString for StockInfo {
    fn to_string(&self) -> String {
        format!(
            "Min: {:.1}, Max: {:.1}, Average: {:.1}, Std: {:.1}",
            self.min, self.max, self.avg, self.std
        )
    }
}

pub struct DataHolder {
    window: Box<dyn Window>,
}

impl DataHolder {
    pub fn new(window: Box<dyn Window>) -> DataHolder {
        DataHolder { window }
    }

    pub fn add_record(&mut self, record: Record) {
        self.window.add_record(record);
    }

    pub fn add_records(&mut self, records: Vec<Record>) {
        self.window.add_records(records);
    }

    fn get_info_of_close_value(records: Vec<Record>) -> StockInfo {
        let mut close_values: Vec<f64> = records.iter().map(|r| r.close).collect();
        close_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let min = *close_values.first().unwrap();
        let max = *close_values.last().unwrap();
        let sum: f64 = close_values.iter().sum();
        let avg = sum / close_values.len() as f64;
        let std = close_values
            .iter()
            .map(|v| (v - avg).powi(2))
            .sum::<f64>()
            .sqrt()
            / close_values.len() as f64;
        StockInfo { min, max, avg, std }
    }

    pub fn update(&mut self) {
        self.window.update();
    }

    pub fn is_updated(&mut self) -> bool {
        self.window.is_updated()
    }

    pub fn get_info(&mut self) -> Result<HashMap<StockKind, StockInfo>> {
        let classified_records = self.window.get_classify_records();
        let mut stock_info: HashMap<StockKind, StockInfo> = HashMap::new();
        for (stock_kind, records) in classified_records {
            let info = Self::get_info_of_close_value(records);
            stock_info.insert(stock_kind, info);
        }
        Ok(stock_info)
    }

    pub fn get_records(&mut self) -> Vec<Record> {
        self.window.get_records()
    }
}
