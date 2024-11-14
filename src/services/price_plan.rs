use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::models::price_plan::{PricePlan, PricePlans};

pub static PRICE_PLANS: LazyLock<Mutex<PricePlans>> = LazyLock::new(|| {
    Mutex::new(HashMap::from([
        (
            "price-plan-0",
            PricePlan {
                supplier: "Dr Evil's Dark Energy",
                rate: 10,
            },
        ),
        (
            "price-plan-1",
            PricePlan {
                supplier: "Power for Everyone",
                rate: 2,
            },
        ),
        (
            "price-plan-2",
            PricePlan {
                supplier: "The Green Eco",
                rate: 1,
            },
        ),
    ]))
});
