use crate::stock::window::{TimeStampedRecord, Window};
use chrono::{Duration, Local, NaiveTime};

pub struct TimeWindow<T: Clone> {
    slide_size: Duration,
    window_size: Duration,
    records: Vec<TimeStampedRecord<T>>,
    update_flag: bool,
    window_length: usize,
    last_update_time: NaiveTime,
}

impl<T: Clone> TimeWindow<T> {
    #[allow(dead_code)]
    pub fn new(slide_size: Duration, window_size: Duration) -> TimeWindow<T> {
        TimeWindow {
            slide_size,
            window_size,
            records: Vec::new(),
            update_flag: false,
            window_length: 0,
            last_update_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        }
    }

    fn binary_search(&self, timestamp: NaiveTime) -> usize {
        self.records
            .as_slice()
            .binary_search_by_key(&timestamp, |tsr| tsr.timestamp)
            .map_or_else(|e| e, |i| i)
    }
}

impl<T: Clone> Window<T> for TimeWindow<T> {
    fn add_record(&mut self, record: T, timestamp: NaiveTime) {
        self.records.push(TimeStampedRecord {
            data: record,
            timestamp,
        });
    }

    fn add_records(&mut self, records: Vec<T>, timestamps: NaiveTime) {
        self.records
            .extend(records.into_iter().map(|r| TimeStampedRecord {
                data: r,
                timestamp: timestamps,
            }));
    }

    fn update(&mut self) {
        let now = Local::now().time();
        if now - self.last_update_time > self.slide_size {
            let id = self.binary_search(Local::now().time() - self.window_size);
            self.records = self.records.split_off(id);
            self.update_flag = true;
            self.window_length = self.records.len();
            self.last_update_time = self.records.last().unwrap().timestamp;
        }
    }

    fn is_updated(&self) -> bool {
        self.update_flag
    }

    fn get_records(&mut self) -> Vec<TimeStampedRecord<T>> {
        self.update_flag = false;
        self.records[..self.window_length].to_vec()
    }
}
