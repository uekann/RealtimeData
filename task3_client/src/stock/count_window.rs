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
