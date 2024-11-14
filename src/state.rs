use axum::extract::FromRef;

use crate::{
    models::readings::Readings,
    services::reading::{ReadingsRepository, ReadingsService},
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub readings_service: Arc<Mutex<ReadingsService>>,
}

impl AppState {
    pub fn new(readings: Readings) -> Self {
        AppState {
            readings_service: Arc::new(Mutex::new(ReadingsService::with_data(readings))),
        }
    }
}

impl FromRef<AppState> for Arc<Mutex<ReadingsService>> {
    fn from_ref(app_state: &AppState) -> Arc<Mutex<ReadingsService>> {
        app_state.readings_service.clone()
    }
}
