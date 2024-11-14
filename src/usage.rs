use crate::{models::readings::Reading, services::price_plan::PRICE_PLANS};
use std::collections::HashMap;

pub fn average(readings: Vec<Reading>) -> f32 {
    let sum = readings.iter().fold(0.0, |acc, &x| acc + x.reading);

    sum / (readings.len() as f32)
}

pub fn time_elapsed_in_hours(readings: Vec<Reading>) -> f32 {
    let mut readings = readings;
    readings.sort_by(|a, b| a.time.cmp(&b.time));

    let first = readings.first().unwrap();
    let last = readings.last().unwrap();
    let seconds = last.time - first.time;
    (seconds as f32 / 3600.0).floor()
}

pub fn usage(readings: Vec<Reading>) -> f32 {
    let average = average(readings.clone());
    let time_elapsed = time_elapsed_in_hours(readings.clone());

    average / time_elapsed
}

pub fn usage_cost(readings: Vec<Reading>, rate: f32) -> f32 {
    usage(readings) * rate
}

pub fn usage_for_all_price_plans(readings: Vec<Reading>) -> Vec<HashMap<String, f32>> {
    PRICE_PLANS
        .lock()
        .unwrap()
        .iter()
        .map(|(key, price_plan)| {
            HashMap::from([(
                key.to_string(),
                usage_cost(readings.clone(), price_plan.rate as f32),
            )])
        })
        .collect()
}
