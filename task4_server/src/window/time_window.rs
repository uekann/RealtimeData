use crate::window::window::Window;
use chrono::{Duration, Local, NaiveTime};
use std::sync::{Arc, Mutex};

type Mut<T> = Arc<Mutex<T>>;

pub struct TimeWindow<T: Clone + Send + 'static> {
    slied_size: Duration,
    window_size: Duration,
    records: Mut<Vec<(NaiveTime, T)>>,
    update_flag: Mut<bool>,
    window_length: Mut<usize>,
    last_update_time: Mut<NaiveTime>,
}

impl<T: Clone + Send> TimeWindow<T> {
    #[allow(dead_code)]
    pub fn new(slied_size: Duration, window_size: Duration) -> TimeWindow<T> {
        TimeWindow {
            slied_size,
            window_size,
            records: Arc::new(Mutex::new(Vec::new())),
            update_flag: Arc::new(Mutex::new(false)),
            window_length: Arc::new(Mutex::new(0)),
            last_update_time: Arc::new(Mutex::new(Local::now().naive_local().time())),
        }
    }

    async fn binary_search(&self, timestamp: NaiveTime) -> usize {
        self.records
            .lock()
            .unwrap()
            .as_slice()
            .binary_search_by_key(&timestamp, |(t, _)| *t)
            .map_or_else(|e| e, |i| i)
    }
}

impl<T: Clone + Send> Window<T> for TimeWindow<T> {
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
        let now = Local::now().time();
        let mut records = self.records.lock().unwrap();
        if now - *self.last_update_time.lock().unwrap() > self.slied_size {
            let id = self.binary_search(now - self.window_size).await;
            records.drain(0..id);
            *self.update_flag.lock().unwrap() = true;
            *self.window_length.lock().unwrap() = records.len();
            *self.last_update_time.lock().unwrap() = records.last().unwrap().0;
        }
    }

    async fn is_updated(&self) -> bool {
        self.update_flag.lock().unwrap().clone()
    }

    async fn get_records(&self) -> Vec<(NaiveTime, T)> {
        // self.update_flag = false;
        // self.records[..self.window_length].to_vec()
        let records = self.records.lock().unwrap();
        {
            let mut flag = self.update_flag.lock().unwrap();
            *flag = false;
        }
        records[..*self.window_length.lock().unwrap()].to_vec()
    }
}
