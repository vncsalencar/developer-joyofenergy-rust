use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Readings = HashMap<String, Vec<Reading>>;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Reading {
    pub time: u32,
    pub reading: f32,
}
