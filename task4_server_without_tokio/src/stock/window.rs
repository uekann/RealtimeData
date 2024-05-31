use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Serialize, Deserialize)]
pub struct TimeStampedRecord<T> {
    #[serde(serialize_with = "serialize_naive_time")]
    pub timestamp: NaiveTime,
    pub data: T,
}

fn serialize_naive_time<S>(value: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = value.format("%H:%M:%S%.3f").to_string();
    serializer.serialize_str(&s)
}

pub trait Window<T> {
    fn add_record(&mut self, data: T, timestamp: NaiveTime);
    fn add_records(&mut self, data: Vec<T>, timestamp: NaiveTime);
    fn is_updated(&self) -> bool;
    fn update(&mut self);
    fn get_records(&mut self) -> Vec<TimeStampedRecord<T>>;
}
