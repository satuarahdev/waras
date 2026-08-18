<div align="center">
  <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" alt="Rust Logo" width="80" height="80" />
  <h1>WARAS</h1>
  <p><b>W</b>hats<b>A</b>pp <b>R</b>ust <b>A</b>PI <b>S</b>erver</p>
  <p><i>The "Sane" (Waras) Way to connect with Official WhatsApp Cloud API.</i></p>

  ![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg?style=flat-square)
  ![Axum](https://img.shields.io/badge/Axum-Web_Framework-blue.svg?style=flat-square)
  ![Docker](https://img.shields.io/badge/Docker-Ready-2496ED.svg?style=flat-square)
  ![Meta](https://img.shields.io/badge/Meta-Official_Cloud_API-0668E1.svg?style=flat-square)
</div>

---

## 🧐 What is WARAS?

Interacting directly with the **Official Meta WhatsApp Cloud API** often gives developers a headache: heavily nested JSON payloads, manual Bearer Token management on every request, and a convoluted Webhook verification process.

**WARAS** is here to make your WhatsApp integration **"Waras"** (an Indonesian word meaning sane & logical). 

Similar to other API gateways, WARAS is a **Middleware** that you install on your server (via Docker). The difference is that WARAS is designed specifically for the **100% Official (Cloud API)** route, built using the blazing-fast and memory-efficient **Rust** programming language.

You simply send a very basic, flat JSON to WARAS, and it swallows all the complexity of encryption, tokens, and Meta's strict formatting behind the scenes.

---

## ✨ Why WARAS?

| Feature | Meta Cloud API (Raw) | WARAS API |
| :--- | :--- | :--- |
| **Banned Risk** | 0% (Official) | **0% (Official)** |
| **Payload Format** | Highly complex & nested | **Simple & Flat JSON** |
| **Request Authentication** | Manual Bearer Token injection | **Automatically handled via `.env`** |
| **Webhook Challenge** | Tedious manual verification | **Automatically answered by WARAS** *(Coming Soon)* |
| **Memory Footprint** | N/A | **< 20 MB RAM (Thanks to Rust & Axum)** |

---

## 🚀 Quick Start (Docker Plug-and-Play)

You don't need to know Rust to use WARAS. Just use Docker!

### 1. Meta Credentials Preparation
Ensure you have an App set up at [Meta for Developers](https://developers.facebook.com/) and prepare:
1. `WhatsApp Phone Number ID`
2. `API Token` (Temporary / Permanent)

### 2. Environment Setup
Clone this repository and copy the environment configuration:
```bash
git clone https://github.com/satuarahdev/waras.git
cd waras
cp .env.example .env
```
Open `.env` and insert your Meta Dashboard credentials:
```env
PORT=3000
WHATSAPP_PHONE_NUMBER_ID=123456789012345
FACEBOOK_API_TOKEN=EAABwzL...
```

### 3. Spin up WARAS
```bash
docker compose up -d
```
WARAS is now running peacefully at `http://localhost:3000`! 🎉

---

## 📚 API Reference

### 📩 Sending a Text Message
Send a simple request to WARAS, and let it deal with Meta's servers.

**`POST /api/message/send`**
```bash
curl -X POST http://localhost:3000/api/message/send \
     -H "Content-Type: application/json" \
     -d '{
           "to": "6281234567890",
           "text": "Hello, this is a test message from WARAS!"
         }'
```
*(Note: The `to` phone number should include the country code without the `+` sign)*

**Success Response:**
```json
{
  "status": "success",
  "message_id": "wamid.HBgL...",
  "error": null
}
```

> 💡 **Tip:** Use the provided [WhatsApp_Cloud_Wrapper.postman_collection.json](./WhatsApp_Cloud_Wrapper.postman_collection.json) to instantly test the endpoints in Postman!

---

## 🗺️ Roadmap

As an open-source project under the [Satuarah.id](https://satuarah.id/) ecosystem, WARAS will continue to evolve:
- [x] **Sender Agent**: Abstraction for sending simple text messages.
- [x] **Webhook Agent**: An endpoint that automatically replies to Meta's Webhook challenges and forwards incoming message events as clean, flat JSON to your app.
- [ ] **Media Agent**: Support for uploading & sending images/documents as easily as attaching a URL.
- [ ] **Template Agent**: Simplified template message dispatching (for OTP/Marketing).

---

## 🤝 Contributing
WARAS is built with love using Rust. If you appreciate its performance and simplicity, please support us by giving a ⭐ **Star** to this repository! 

Pull Requests (PRs) for new features or bug fixes are highly welcome.
