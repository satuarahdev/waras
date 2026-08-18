mod handlers;
mod models;
mod routes;
mod whatsapp_client;

use std::env;
use whatsapp_client::AppState;

#[tokio::main]
async fn main() {
    // Load environment variables from .env if present
    let _ = dotenvy::dotenv();

    // Initialize tracing (logging) with GMT+7 (WIB / Asia/Jakarta)
    let offset = time::UtcOffset::from_hms(7, 0, 0).unwrap_or(time::UtcOffset::UTC);
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        offset,
        time::format_description::well_known::Rfc3339,
    );
    tracing_subscriber::fmt()
        .with_timer(timer)
        .init();

    // Get port from env
    let port = env::var("PORT").expect("PORT must be set in .env");
    let addr = format!("0.0.0.0:{}", port);

    // Initialize application state
    let state = AppState::new();

    // Create the Axum router
    let app = routes::create_router(state);

    // Bind and start the server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server started on {}!", addr);
    tracing::info!("Available endpoints:");
    tracing::info!(" - POST /api/message/send");
    tracing::info!(" - GET  /api/webhook (Verification)");
    tracing::info!(" - POST /api/webhook (Events)");

    axum::serve(listener, app).await.unwrap();
}
