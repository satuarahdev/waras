use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::{message, webhook};
use crate::whatsapp_client::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health endpoints
        .route("/", get(|| async { axum::Json(serde_json::json!({"status": "running", "service": "waras"})) }))
        .route("/healthz", get(|| async { axum::Json(serde_json::json!({"status": "ok"})) }))
        // Message endpoints
        .route("/api/message/send", post(message::send_message))
        .route("/api/message/read", post(message::mark_as_read))
        .route("/api/media/send", post(message::send_media))
        .route("/api/media/upload", post(message::upload_media))
        .route("/api/media/download", get(message::download_media))
        // Webhook endpoints
        .route("/api/webhook", get(webhook::verify_webhook).post(webhook::handle_webhook_event))
        .layer(axum::extract::DefaultBodyLimit::max(100 * 1024 * 1024))
        // Provide the state to all routes
        .with_state(state)
}
