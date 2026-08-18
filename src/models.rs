use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub to: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub status: String,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookVerificationQuery {
    #[serde(rename = "hub.mode")]
    pub hub_mode: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub hub_challenge: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub hub_verify_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppWebhookPayload {
    pub object: Option<String>,
    pub entry: Option<Vec<WhatsAppEntry>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppEntry {
    pub id: Option<String>,
    pub changes: Option<Vec<WhatsAppChange>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppChange {
    pub field: Option<String>,
    pub value: Option<WhatsAppValue>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppValue {
    pub messaging_product: Option<String>,
    pub metadata: Option<WhatsAppMetadata>,
    pub contacts: Option<Vec<WhatsAppContact>>,
    pub messages: Option<Vec<WhatsAppMessage>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppMetadata {
    pub display_phone_number: Option<String>,
    pub phone_number_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppContact {
    pub profile: Option<WhatsAppProfile>,
    pub wa_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppProfile {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppMessage {
    pub from: Option<String>,
    pub id: Option<String>,
    pub timestamp: Option<String>,
    #[serde(rename = "type")]
    pub message_type: Option<String>,
    pub text: Option<WhatsAppText>,
    pub document: Option<WhatsAppMedia>,
    pub image: Option<WhatsAppMedia>,
    pub video: Option<WhatsAppMedia>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppText {
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhatsAppMedia {
    pub id: Option<String>,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub caption: Option<String>,
}
