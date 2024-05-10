use crate::stock::record::Record;
use crate::stock::window::Window;
use chrono::{Duration, Local};

pub struct TimeWindow {
    window_size: Duration,
    records: Vec<Record>,
}

impl TimeWindow {
    pub fn new(window_size: Duration) -> TimeWindow {
        TimeWindow {
            window_size,
            records: Vec::new(),
        }
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
        let id = self
            .records
            .as_slice()
            .binary_search_by_key(
                &(Local::now().naive_local().time() - self.window_size),
                |r| r.timestamp,
            )
            .map_or_else(|e| e, |i| i);

        self.records = self.records.split_off(id);
    }

    fn get_records(&self) -> Vec<Record> {
        self.records.clone()
    }
}
