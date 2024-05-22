use crate::stock::record::Record;
use crate::stock::window::Window;
use chrono::{Duration, Local, NaiveTime};

pub struct TimeWindow {
    slied_size: Duration,
    window_size: Duration,
    records: Vec<Record>,
    update_flag: bool,
    window_length: usize,
    last_update_time: NaiveTime,
}

impl TimeWindow {
    #[allow(dead_code)]
    pub fn new(slied_size: Duration, window_size: Duration) -> TimeWindow {
        TimeWindow {
            slied_size,
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
            .binary_search_by_key(&timestamp, |r| r.timestamp)
            .map_or_else(|e| e, |i| i)
    }
}

impl Window for TimeWindow {
    fn add_record(&mut self, record: Record) {
        self.records.push(record);
    }

    fn add_records(&mut self, records: Vec<Record>) {
        self.records.extend(records);
    }

    fn update(&mut self) {
        let now = Local::now().time();
        if now - self.last_update_time > self.slied_size {
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

    fn get_records(&mut self) -> Vec<Record> {
        self.update_flag = false;
        self.records[..self.window_length].to_vec()
    }
}
