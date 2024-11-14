use crate::models::readings::{Reading, Readings};

pub struct ReadingsService {
    pub readings: Readings,
}

pub trait ReadingsRepository {
    fn with_data(readings: Readings) -> Self;
    fn get_readings(&mut self, meter_id: &String) -> Option<&mut Vec<Reading>>;
    fn push_readings(&mut self, meter_id: String, readings: Vec<Reading>) -> &Vec<Reading>;
    fn has_readings(&self, meter_id: &String) -> bool;
}

impl ReadingsRepository for ReadingsService {
    fn with_data(readings: Readings) -> Self {
        Self { readings }
    }

    fn get_readings(&mut self, meter_id: &String) -> Option<&mut Vec<Reading>> {
        if !self.readings.contains_key(meter_id) {
            return None;
        }

        self.readings.get_mut(meter_id)
        // Some(&self.readings[&meter_id])
    }

    fn push_readings(&mut self, meter_id: String, readings: Vec<Reading>) -> &Vec<Reading> {
        let entry = self.readings.entry(meter_id).or_default();
        entry.extend(readings);
        &*entry
    }

    fn has_readings(&self, meter_id: &String) -> bool {
        self.readings.contains_key(meter_id)
    }
}
