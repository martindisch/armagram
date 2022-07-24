use reqwest::blocking::Client;
use std::collections::HashMap;

/// Thin Telegram client for sending messages to a single chat via bot.
pub struct Telegram {
    endpoint: String,
    chat_id: String,
    client: Client,
}

impl Telegram {
    /// Initializes a new instance.
    pub fn new(token: String, chat_id: String) -> Self {
        let client = Client::new();

        Self {
            endpoint: format!(
                "https://api.telegram.org/bot{token}/sendMessage"
            ),
            chat_id,
            client,
        }
    }

    /// Sends the given message to the configured chat.
    pub fn send(&self, message: &str) {
        self.client
            .post(&self.endpoint)
            .form(&HashMap::from([
                ("chat_id", self.chat_id.as_ref()),
                ("text", message),
            ]))
            .send()
            // If Telegram/network fails there's not really anything useful
            // to do, so just swallow the error
            .ok();
    }
}
