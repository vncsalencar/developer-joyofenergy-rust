mod helpers;
mod models;
mod routes;
mod services;
mod shutdown;
mod state;
mod usage;

use helpers::readings::generate_readings;
use routes::get_router;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8081".to_owned());
    let addr = format!("{host}:{port}");

    let listener = TcpListener::bind(&addr).await.unwrap();
    let readings = generate_readings();
    let app = get_router().with_state(state::AppState::new(readings));

    println!("🚀 app listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
