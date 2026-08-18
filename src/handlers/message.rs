use axum::{
    extract::{Multipart, State},
    Json,
};
use serde::Deserialize;
use crate::models::{SendMessageRequest, SendMessageResponse};
use crate::whatsapp_client::AppState;

#[derive(Deserialize)]
pub struct MarkReadRequest {
    pub message_id: String,
}

#[derive(Deserialize)]
pub struct SendMediaRequest {
    pub to: String,
    pub media_id: String,
    pub media_type: String,
    pub caption: Option<String>,
    pub filename: Option<String>,
}

pub async fn send_message(
    State(state): State<AppState>,
    Json(payload): Json<SendMessageRequest>,
) -> Json<SendMessageResponse> {
    match state.send_message(&payload.to, &payload.text).await {
        Ok(msg_id) => Json(SendMessageResponse {
            status: "success".to_string(),
            message_id: Some(msg_id),
            error: None,
        }),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Json(SendMessageResponse {
                status: "error".to_string(),
                message_id: None,
                error: Some(e),
            })
        },
    }
}

pub async fn mark_as_read(
    State(state): State<AppState>,
    Json(payload): Json<MarkReadRequest>,
) -> Json<SendMessageResponse> {
    match state.mark_as_read(&payload.message_id).await {
        Ok(_) => Json(SendMessageResponse {
            status: "success".to_string(),
            message_id: None,
            error: None,
        }),
        Err(e) => {
            tracing::error!("Failed to mark message as read: {}", e);
            Json(SendMessageResponse {
                status: "error".to_string(),
                message_id: None,
                error: Some(e),
            })
        },
    }
}

pub async fn send_media(
    State(state): State<AppState>,
    Json(payload): Json<SendMediaRequest>,
) -> Json<SendMessageResponse> {
    match state.send_media(&payload.to, &payload.media_id, &payload.media_type, payload.caption, payload.filename).await {
        Ok(msg_id) => Json(SendMessageResponse {
            status: "success".to_string(),
            message_id: Some(msg_id),
            error: None,
        }),
        Err(e) => {
            tracing::error!("Failed to send media: {}", e);
            Json(SendMessageResponse {
                status: "error".to_string(),
                message_id: None,
                error: Some(e),
            })
        },
    }
}

pub async fn upload_media(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Json<SendMessageResponse> {
    let mut file_bytes = Vec::new();
    let mut mime_type = String::from("application/octet-stream");
    let mut filename = String::from("file");

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" || name == "data" {
            if let Some(fn_name) = field.file_name() {
                filename = fn_name.to_string();
            }
            if let Some(ct) = field.content_type() {
                mime_type = ct.to_string();
            }
            file_bytes = field.bytes().await.unwrap_or_default().to_vec();
        }
    }

    if file_bytes.is_empty() {
        return Json(SendMessageResponse {
            status: "error".to_string(),
            message_id: None,
            error: Some("No file provided".to_string()),
        });
    }

    match state.upload_media(file_bytes, &mime_type, &filename).await {
        Ok(media_id) => Json(SendMessageResponse {
            status: "success".to_string(),
            message_id: Some(media_id),
            error: None,
        }),
        Err(e) => {
            tracing::error!("Failed to upload media: {}", e);
            Json(SendMessageResponse {
                status: "error".to_string(),
                message_id: None,
                error: Some(e),
            })
        },
    }
}

#[derive(Deserialize)]
pub struct DownloadMediaParams {
    pub media_id: String,
}

pub async fn download_media(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<DownloadMediaParams>,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    match state.download_media(&params.media_id).await {
        Ok((bytes, mime)) => {
            let res = axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header(axum::http::header::CONTENT_TYPE, mime)
                .body(axum::body::Body::from(bytes))
                .unwrap();
            Ok(res)
        }
        Err(e) => {
            tracing::error!("Download media error: {}", e);
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }
}
