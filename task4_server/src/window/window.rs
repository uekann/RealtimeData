use chrono::NaiveTime;
use tokio;

pub trait Window<T>: Send + Sync + 'static {
    async fn add_record(&self, data: T, timestamp: NaiveTime);
    async fn add_records(&self, data: Vec<T>, timestamp: NaiveTime);
    async fn is_updated(&self) -> bool;
    async fn update(&self);
    async fn get_records(&self) -> Vec<(NaiveTime, T)>;
}
