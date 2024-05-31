use crate::{stock::record, window::window::Window};
use chrono::NaiveTime;
use std::sync::{Arc, Mutex};

type Mut<T> = Arc<Mutex<T>>;

pub struct CountWindow<T: Clone + Send + 'static> {
    slide_size: usize,
    window_size: usize,
    records: Mut<Vec<(NaiveTime, T)>>,
    update_flag: Mut<bool>,
}

impl<T: Clone + Send> CountWindow<T> {
    #[allow(dead_code)]
    pub fn new(slide_size: usize, window_size: usize) -> CountWindow<T> {
        CountWindow {
            update_flag: Arc::new(Mutex::new(false)),
            slide_size,
            window_size,
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<T: Clone + Send> Window<T> for CountWindow<T> {
    async fn add_record(&self, record: T, timestamp: NaiveTime) {
        self.records.lock().unwrap().push((timestamp, record));
    }

    async fn add_records(&self, records: Vec<T>, timestamps: NaiveTime) {
        self.records
            .lock()
            .unwrap()
            .extend(records.into_iter().map(|r| (timestamps, r)));
    }

    async fn update(&self) {
        let mut records = self.records.lock().unwrap();
        if (records.len() as i64) - (self.window_size as i64) >= self.slide_size as i64 {
            let len = records.len();
            records.drain(0..len - self.window_size);
            *self.update_flag.lock().unwrap() = true;
        }
    }

    async fn is_updated(&self) -> bool {
        // self.update_flag || self.records.len() < self.window_size
        let len = self.records.lock().unwrap().len();
        let flag = *self.update_flag.lock().unwrap();
        flag || len < self.window_size
    }

    async fn get_records(&self) -> Vec<(NaiveTime, T)> {
        // self.update_flag = false;
        // self.records[..std::cmp::min(self.window_size, self.records.len())].to_vec()
        let records = self.records.lock().unwrap();
        {
            let mut flag = self.update_flag.lock().unwrap();
            *flag = false;
        }
        let len = records.len();
        records[..std::cmp::min(self.window_size, len)].to_vec()
    }
}
