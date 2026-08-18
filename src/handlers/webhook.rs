use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    models::WebhookVerificationQuery,
    whatsapp_client::AppState,
};

pub async fn verify_webhook(
    State(state): State<AppState>,
    Query(query): Query<WebhookVerificationQuery>,
) -> impl IntoResponse {
    if let (Some(mode), Some(token), Some(challenge)) =
        (query.hub_mode, query.hub_verify_token, query.hub_challenge)
    {
        if mode == "subscribe" && token == state.webhook_verify_token {
            tracing::info!("Webhook verified successfully.");
            return (StatusCode::OK, challenge);
        }
    }

    tracing::warn!("Webhook verification failed.");
    (StatusCode::FORBIDDEN, "Verification failed".to_string())
}

pub async fn handle_webhook_event(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    body: String,
) -> impl IntoResponse {
    tracing::info!("--- NEW WEBHOOK EVENT ---");
    tracing::info!("Headers: {:?}", headers);
    
    let payload: serde_json::Value = match serde_json::from_str(&body) {
        Ok(p) => {
            tracing::info!("Payload:\n{}", serde_json::to_string_pretty(&p).unwrap_or_else(|_| body.clone()));
            p
        },
        Err(e) => {
            tracing::error!("Failed to parse webhook JSON: {}\nRaw Body: {}", e, body);
            return StatusCode::BAD_REQUEST;
        }
    };
    
    // Spawn a background task to forward the payload so we don't block the 200 OK response to Meta
    tokio::spawn(async move {
        // Forward to the configured webhook URL
        let forward_url = std::env::var("WEBHOOK_FORWARD_URL").unwrap_or_else(|_| "http://backend-rust:25333/api/v1/webhook/whatsapp".to_string());
        
        match state.http_client.post(&forward_url).json(&payload).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    tracing::info!("Successfully forwarded to {}: Status {}", forward_url, status);
                } else {
                    let err_body = resp.text().await.unwrap_or_else(|_| "<failed to read body>".to_string());
                    tracing::error!("Error response from {}: Status {}\nBody: {}", forward_url, status, err_body);
                }
            }
            Err(e) => {
                tracing::error!("Network error forwarding to {}: {}", forward_url, e);
            }
        }
        tracing::info!("--- END WEBHOOK EVENT ---");
    });

    // Acknowledge receipt to Meta immediately
    StatusCode::OK
}
