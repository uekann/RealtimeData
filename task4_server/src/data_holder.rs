use crate::stock::record::{Record, StockKind};
use crate::window::window::Window;
use anyhow::Result;
use chrono::{Local, NaiveTime};
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

pub struct DataHolder<W: Window<Record>> {
    window: W,
}

#[allow(dead_code)]
impl<W: Window<Record>> DataHolder<W> {
    pub fn new(window: W) -> DataHolder<W> {
        DataHolder { window }
    }

    pub async fn add_record(&mut self, record: Record) {
        self.window
            .add_record(record, Local::now().naive_local().time())
            .await;
    }

    pub async fn add_records(&mut self, records: Vec<Record>) {
        self.window
            .add_records(records, Local::now().naive_local().time())
            .await;
    }

    async fn get_info_of_close_value(records: Vec<Record>) -> StockInfo {
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

    pub async fn update(&mut self) {
        self.window.update().await;
    }

    pub async fn is_updated(&mut self) -> bool {
        self.window.is_updated().await
    }

    pub async fn get_info(&mut self) -> Result<HashMap<StockKind, StockInfo>> {
        let records = self.window.get_records().await;
        let mut classified_records = HashMap::new();
        for record in records {
            let key = record.1.stock.clone();
            let value = record.1;
            classified_records
                .entry(key)
                .or_insert(Vec::new())
                .push(value);
        }
        let mut stock_info: HashMap<StockKind, StockInfo> = HashMap::new();
        for (stock_kind, records) in classified_records {
            let info = Self::get_info_of_close_value(records).await;
            stock_info.insert(stock_kind, info);
        }
        Ok(stock_info)
    }

    pub async fn get_records(&mut self) -> Vec<(NaiveTime, Record)> {
        self.window.get_records().await
    }
}
