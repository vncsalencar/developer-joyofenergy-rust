use crate::{
    models::readings::Reading,
    services::reading::{ReadingsRepository, ReadingsService},
    state,
    usage::usage_for_all_price_plans,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub fn get_router() -> Router<state::AppState> {
    Router::new()
        .route(
            "/readings/read/:smart_meter_id",
            get(get_readings_by_smart_meter_id),
        )
        .route("/readings/store", post(store_reading))
        .route(
            "/price-plans/recommend/:smart_meter_id",
            get(get_price_plan_recommendation_by_smart_meter_id),
        )
        .route(
            "/price-plans/compare-all/:smart_meter_id",
            get(compare_price_plans_by_smart_meter_id),
        )
}

async fn get_readings_by_smart_meter_id(
    State(readings_service): State<Arc<Mutex<ReadingsService>>>,
    Path(smart_meter_id): Path<String>,
) -> impl IntoResponse {
    if let Some(readings) = readings_service
        .lock()
        .unwrap()
        .get_readings(&smart_meter_id)
    {
        return (StatusCode::OK, Json(readings)).into_response();
    }

    (
        StatusCode::NOT_FOUND,
        "Data for supplied smart meter id not found",
    )
        .into_response()
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NewReading {
    smart_meter_id: String,
    electricity_readings: Vec<Reading>,
}

async fn store_reading(
    State(readings_service): State<Arc<Mutex<ReadingsService>>>,
    Json(new_reading): Json<NewReading>,
) -> impl IntoResponse {
    let mut readings = readings_service.lock().unwrap();

    if !readings.has_readings(&new_reading.smart_meter_id) {
        return (
            StatusCode::NOT_FOUND,
            "Cannot store readings for unknown smart meter id",
        )
            .into_response();
    }

    if let Some(readings) = readings.get_readings(&new_reading.smart_meter_id) {
        readings.extend(new_reading.electricity_readings);
        return (StatusCode::OK, Json(readings)).into_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to store readings",
    )
        .into_response()
}

#[derive(Deserialize)]
struct PricePlanRecommendationQuery {
    limit: Option<u32>,
}

async fn get_price_plan_recommendation_by_smart_meter_id(
    State(readings_service): State<Arc<Mutex<ReadingsService>>>,
    Path(smart_meter_id): Path<String>,
    Query(params): Query<PricePlanRecommendationQuery>,
) -> impl IntoResponse {
    let mut readings = readings_service.lock().unwrap();

    let extract_cost =
        |comparison: &HashMap<String, f32>| comparison.values().next().unwrap().to_owned();
    let readings = readings.get_readings(&smart_meter_id).unwrap().to_owned();
    let mut price_plan_comparisons = usage_for_all_price_plans(readings);
    price_plan_comparisons.sort_by(|a, b| {
        extract_cost(a)
            .partial_cmp(&extract_cost(b))
            .unwrap_or(Ordering::Equal)
    });

    if params.limit.is_some() {
        let comparisons = price_plan_comparisons
            .into_iter()
            .take(params.limit.unwrap() as usize)
            .collect::<Vec<HashMap<String, f32>>>();
        return (StatusCode::OK, Json(comparisons)).into_response();
    }

    (StatusCode::OK, Json(price_plan_comparisons)).into_response()
}

async fn compare_price_plans_by_smart_meter_id(
    State(readings_service): State<Arc<Mutex<ReadingsService>>>,
    Path(smart_meter_id): Path<String>,
) -> impl IntoResponse {
    let mut readings = readings_service.lock().unwrap();
    let readings = readings.get_readings(&smart_meter_id).unwrap().to_owned();
    let price_plan_comparisons = usage_for_all_price_plans(readings);

    let json = json!({
        "smartMeterId": smart_meter_id,
        "pricePlanComparisons": price_plan_comparisons
    });

    (StatusCode::OK, Json(json)).into_response()
}
