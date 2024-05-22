use chrono::NaiveTime;

pub trait Window<T> {
    fn add_record(&mut self, data: T, timestamp: NaiveTime);
    fn add_records(&mut self, data: Vec<T>, timestamp: NaiveTime);
    fn is_updated(&self) -> bool;
    fn update(&mut self);
    fn get_records(&mut self) -> Vec<(NaiveTime, T)>;
}
