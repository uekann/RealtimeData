use crate::Record;
use chrono::Duration;

pub struct TimeWindow {
    window_size: Duration,
    records: Vec<Record>,
}
