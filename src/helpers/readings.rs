use crate::models::readings::Reading;
use rand::Rng;
use std::collections::HashMap;

const READINGS_LENGTH: u32 = 5;
const START: u32 = 1607686125;
const HOUR: u32 = 3600;

fn generate_single_meter_reading() -> Vec<Reading> {
    let readings_length = rand::thread_rng().gen_range(1..=20);

    (0..readings_length)
        .map(|idx| {
            let time = START - idx * HOUR;
            let reading = rand::thread_rng().gen_range(0.0..2.0);

            Reading { time, reading }
        })
        .collect::<Vec<Reading>>()
}

pub fn generate_readings() -> HashMap<String, Vec<Reading>> {
    (0..READINGS_LENGTH)
        .map(|idx| {
            let meter_name = format!("smart-meter-{}", idx);
            let readings = generate_single_meter_reading();

            (meter_name, readings)
        })
        .collect::<HashMap<String, Vec<Reading>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_amount_of_readings() {
        let readings = generate_readings();
        assert_eq!(readings.len(), READINGS_LENGTH as usize);
    }
}
