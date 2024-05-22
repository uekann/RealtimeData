use chrono::NaiveTime;

use crate::window::window::Window;

pub struct CountWindow<T: Clone> {
    slide_size: usize,
    window_size: usize,
    records: Vec<(NaiveTime, T)>,
    update_flag: bool,
}

impl<T: Clone> CountWindow<T> {
    #[allow(dead_code)]
    pub fn new(slide_size: usize, window_size: usize) -> CountWindow<T> {
        CountWindow {
            update_flag: false,
            slide_size,
            window_size,
            records: Vec::new(),
        }
    }
}

impl<T: Clone> Window<T> for CountWindow<T> {
    fn add_record(&mut self, record: T, timestamp: NaiveTime) {
        self.records.push((timestamp, record));
    }

    fn add_records(&mut self, records: Vec<T>, timestamps: NaiveTime) {
        self.records
            .extend(records.into_iter().map(|r| (timestamps, r)));
    }

    fn update(&mut self) {
        if (self.records.len() as i64) - (self.window_size as i64) >= self.slide_size as i64 {
            self.records = self
                .records
                .split_off(self.records.len() - self.window_size);
            self.update_flag = true;
        }
    }

    fn is_updated(&self) -> bool {
        self.update_flag || self.records.len() < self.window_size
    }

    fn get_records(&mut self) -> Vec<(NaiveTime, T)> {
        self.update_flag = false;
        self.records[..std::cmp::min(self.window_size, self.records.len())].to_vec()
    }
}
