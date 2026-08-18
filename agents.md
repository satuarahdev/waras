# Vision & System Architecture

This document clearly outlines the **direction, purpose, and role** of the *WhatsApp Cloud API Wrapper* (WARAS) project.

## 1. The Problem
Facebook/Meta provides the Official WhatsApp Cloud API, but their system is notoriously complex for the average developer:
1. Extremely deep and nested JSON request formats.
2. Webhook setup requires a convoluted *challenge token* verification process.
3. Incoming message formats (from webhooks) are bloated with unnecessary metadata.

## 2. The Goal
This application is built to be the **"Meta API Tamer" (API Gateway / Middleware)**. The primary goal is that developers simply install this Docker image, populate the `.env` file, and instantly have a local WhatsApp API on their server that is:
- Exceptionally easy to call (simple JSON formats).
- 100% Official & Safe (utilizing Meta's direct infrastructure).
- Free from the hassle of injecting Bearer Tokens into every request, as the app handles it behind the scenes.

## 3. System Components (The Agents)

To handle everything seamlessly, this application is conceptually divided into several asynchronous "Agents" (Modules):

### A. The Sender Agent (Currently Active)
- **Function:** Receives message dispatch requests from the user's application using a radically simplified format.
- **Behind the scenes:** Constructs a valid JSON structure according to Meta's strict standards, injects the Bearer Token from `.env`, and fires it to the Graph API.

### B. The Webhook Agent (Next on Roadmap)
- **Function:** Acts as the frontline guard facing Meta's servers.
- **Behind the scenes:** 
  1. Automatically answers the *Challenge Request* when users link the Webhook in the Meta dashboard.
  2. Receives incoming message events (text, images, read/delivered statuses).
  3. Parses and simplifies the complex incoming JSON into a clean, flat format.
  4. Forwards this tidy JSON to the user's application URL.

### C. The Media Agent (Future Roadmap)
- **Function:** Handles media uploads and downloads.
- **Behind the scenes:** Receives a public URL or *form-data* from the user, uploads it to Meta, retrieves the Media ID, and then sends it as an image/document message.

## 4. Architectural Topology

```text
[User's Business Application] (PHP/NodeJS/Python)
       |
       | (Fast & Simple JSON via Localhost:3000)
       V
+-------------------------------------------------+
|               WHATSAPP RUST WRAPPER             |
|                                                 |
|  [Sender Agent]  [Webhook Agent]  [Media Agent] |
+-------------------------------------------------+
       |
       | (Complex & Secure JSON via HTTPS + Bearer Token)
       V
[Meta Server / Facebook Graph API]
       |
       V
[Customer's Phone]
```
