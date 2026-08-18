use reqwest::Client;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub api_token: String,
    pub phone_number_id: String,
    pub webhook_verify_token: String,
}

impl AppState {
    pub fn new() -> Self {
        let api_token = env::var("FACEBOOK_API_TOKEN").expect("FACEBOOK_API_TOKEN must be set");
        let phone_number_id = env::var("WHATSAPP_PHONE_NUMBER_ID").expect("WHATSAPP_PHONE_NUMBER_ID must be set");
        let webhook_verify_token = env::var("WEBHOOK_VERIFY_TOKEN").unwrap_or_else(|_| "my_secret_token".to_string());

        Self {
            http_client: Client::new(),
            api_token,
            phone_number_id,
            webhook_verify_token,
        }
    }

    pub async fn send_message(&self, to: &str, text: &str) -> Result<String, String> {
        let url = format!(
            "https://graph.facebook.com/v20.0/{}/messages",
            self.phone_number_id
        );

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": "text",
            "text": {
                "preview_url": false,
                "body": text
            }
        });

        let res = self.http_client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            let message_id = body["messages"][0]["id"].as_str().unwrap_or("unknown_id").to_string();
            Ok(message_id)
        } else {
            let error_body = res.text().await.unwrap_or_default();
            Err(format!("API Error: {}", error_body))
        }
    }

    pub async fn mark_as_read(&self, message_id: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v20.0/{}/messages",
            self.phone_number_id
        );

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "status": "read",
            "message_id": message_id
        });

        let res = self.http_client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            let error_body = res.text().await.unwrap_or_default();
            Err(format!("Mark Read Error: {}", error_body))
        }
    }

    pub async fn upload_media(&self, file_bytes: Vec<u8>, mime_type: &str, filename: &str) -> Result<String, String> {
        let url = format!(
            "https://graph.facebook.com/v20.0/{}/media",
            self.phone_number_id
        );

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(filename.to_string())
            .mime_str(mime_type)
            .map_err(|e| e.to_string())?;

        let form = reqwest::multipart::Form::new()
            .text("messaging_product", "whatsapp")
            .part("file", part);

        let res = self.http_client
            .post(&url)
            .bearer_auth(&self.api_token)
            .multipart(form)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            let media_id = body["id"].as_str().unwrap_or_default().to_string();
            Ok(media_id)
        } else {
            let error_body = res.text().await.unwrap_or_default();
            Err(format!("Upload Media Error: {}", error_body))
        }
    }

    pub async fn send_media(
        &self,
        to: &str,
        media_id: &str,
        media_type: &str,
        caption: Option<String>,
        filename: Option<String>,
    ) -> Result<String, String> {
        let url = format!(
            "https://graph.facebook.com/v20.0/{}/messages",
            self.phone_number_id
        );

        let mut media_payload = serde_json::json!({
            "id": media_id,
        });

        if let Some(c) = caption {
            if !c.is_empty() {
                media_payload["caption"] = serde_json::Value::String(c);
            }
        }

        if media_type == "document" {
            if let Some(f) = filename {
                if !f.is_empty() {
                    media_payload["filename"] = serde_json::Value::String(f);
                }
            }
        }

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": media_type,
            media_type: media_payload
        });

        let res = self.http_client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            let message_id = body["messages"][0]["id"].as_str().unwrap_or("unknown_id").to_string();
            Ok(message_id)
        } else {
            let error_body = res.text().await.unwrap_or_default();
            Err(format!("Send Media Error: {}", error_body))
        }
    }

    pub async fn download_media(&self, media_id: &str) -> Result<(Vec<u8>, String), String> {
        let url = format!("https://graph.facebook.com/v20.0/{}", media_id);
        let res = self.http_client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(format!("Failed to get media URL: {}", res.text().await.unwrap_or_default()));
        }

        let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        let download_url = body.get("url").and_then(|u| u.as_str()).ok_or("Media URL missing")?;
        let mime_type = body.get("mime_type").and_then(|m| m.as_str()).unwrap_or("application/octet-stream").to_string();

        let file_res = self.http_client
            .get(download_url)
            .bearer_auth(&self.api_token)
            .header("User-Agent", "facebookexternalua")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !file_res.status().is_success() {
            return Err("Failed to download media binary from Meta".to_string());
        }

        let bytes = file_res.bytes().await.map_err(|e| e.to_string())?.to_vec();
        Ok((bytes, mime_type))
    }
}
