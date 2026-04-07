/*!
 * API Client Module
 *
 * Corresponds to: src/services/api/claude.ts
 *
 * Main features:
 * - Call Anthropic Messages API
 * - Handle streaming responses (Server-Sent Events)
 * - Manage conversation history
 */

use crate::types::*;
use anyhow::{Context, Result};
use bytes::Bytes;
use futures::stream::StreamExt;
use reqwest::Client;
use serde_json::json;
use std::io::{self, Write};

/// Claude API client
pub struct ClaudeClient {
    api_key: String,
    client: Client,
    base_url: String,
    model: String,
    max_tokens: u32,
}

impl ClaudeClient {
    /// Create a new client instance
    /// https://lab.iwhalecloud.com/gpt-proxy/anthropic
    /// https://api.anthropic.com/v1
    pub fn new(api_key: &str) -> Result<Self> {
        Ok(Self {
            api_key: api_key.to_string(),
            client: Client::new(),
            base_url: "https://lab.iwhalecloud.com/gpt-proxy/anthropic".to_string(),
            model: "claude-4.5-sonnet".to_string(),
            max_tokens: 4096,
        })
    }

    /// Query Claude API (streaming response)
    ///
    /// Corresponds to: src/services/api/claude.ts:864
    /// ```typescript
    /// await anthropic.beta.messages.create({
    ///   model: 'claude-opus-4',
    ///   max_tokens: 4096,
    ///   messages: messages,
    ///   stream: true
    /// })
    /// ```
    pub async fn query_streaming(
        &self,
        prompt: &str,
        history: &[Message],
    ) -> Result<String> {
        // Build messages array
        let mut messages: Vec<serde_json::Value> = history
            .iter()
            .map(|msg| {
                json!({
                    "role": msg.role,
                    "content": msg.content
                })
            })
            .collect();

        // Add current user message
        messages.push(json!({
            "role": "user",
            "content": prompt
        }));

        // Build request body
        let body = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": messages,
            "stream": true
        });

        // Send request
        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("API request failed")?;

        // Check response status
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("API returned error {}: {}", status, error_text);
        }

        // Handle streaming response
        let mut full_response = String::new();
        let mut stream = response.bytes_stream();

        // Process Server-Sent Events (SSE) stream
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Failed to read stream data")?;
            let text = String::from_utf8_lossy(&chunk);

            // Parse SSE format: "data: {...}\n\n"
            for line in text.lines() {
                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        break;
                    }

                    // Parse JSON event
                    if let Ok(event) = serde_json::from_str::<StreamEvent>(data) {
                        match event.event_type.as_str() {
                            "content_block_delta" => {
                                if let Some(delta) = event.delta {
                                    if delta.delta_type == "text_delta" {
                                        if let Some(text) = delta.text {
                                            // Print to terminal in real-time
                                            print!("{}", text);
                                            io::stdout().flush().ok();
                                            full_response.push_str(&text);
                                        }
                                    }
                                }
                            }
                            "message_start" => {
                                // Message started
                            }
                            "message_stop" => {
                                // Message ended
                                break;
                            }
                            _ => {
                                // Other event types
                            }
                        }
                    }
                }
            }
        }

        Ok(full_response)
    }

    /// Query Claude API (non-streaming, returns all at once)
    pub async fn query(&self, prompt: &str, history: &[Message]) -> Result<ApiResponse> {
        let mut messages: Vec<serde_json::Value> = history
            .iter()
            .map(|msg| {
                json!({
                    "role": msg.role,
                    "content": msg.content
                })
            })
            .collect();

        messages.push(json!({
            "role": "user",
            "content": prompt
        }));

        let body = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": messages,
            "stream": false
        });

        let response = self
            .client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("API request failed")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("API returned error {}: {}", status, error_text);
        }

        let api_response: ApiResponse = response.json().await.context("Failed to parse response")?;
        Ok(api_response)
    }

    /// Set model
    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }

    /// Set max tokens
    pub fn set_max_tokens(&mut self, max_tokens: u32) {
        self.max_tokens = max_tokens;
    }
}
