use std::collections::HashMap;

pub type PricePlans = HashMap<&'static str, PricePlan>;

pub struct PricePlan {
    pub supplier: &'static str,
    pub rate: u32,
}
