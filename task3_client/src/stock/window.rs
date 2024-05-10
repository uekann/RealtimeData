use crate::stock::record::Record;

pub trait Window {
    fn add_record(&mut self, record: Record);
    fn add_recors(&mut self, records: Vec<Record>);
    fn update(&mut self);
    fn get_records(&self) -> Vec<Record>;
}
