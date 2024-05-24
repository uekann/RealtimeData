use chrono::NaiveTime;
use tokio;

pub trait Window<T> {
    async fn add_record(&mut self, data: T, timestamp: NaiveTime);
    async fn add_records(&mut self, data: Vec<T>, timestamp: NaiveTime);
    async fn is_updated(&self) -> bool;
    async fn update(&mut self);
    async fn get_records(&mut self) -> Vec<(NaiveTime, T)>;
}
