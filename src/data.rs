use std::{sync::Arc, time::Instant};

use teloxide::{
    prelude::*,
    types::{Message, ParseMode},
};

use crate::helpers::config::Config;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Data {
    pub config: Arc<Config>,
    pub bot_username: String,
    pub started_at: Instant,
}

pub struct Context {
    pub bot: Bot,
    pub message: Message,
    pub data: Arc<Data>,
}

impl Context {
    pub fn new(bot: Bot, message: Message, data: Arc<Data>) -> Self {
        Self { bot, message, data }
    }

    pub fn author_id(&self) -> Option<u64> {
        self.message.from.as_ref().map(|user| user.id.0)
    }

    pub fn author_mention(&self) -> String {
        match self.message.from.as_ref() {
            Some(user) => match &user.username {
                Some(username) => format!("@{username}"),
                None => user.first_name.clone(),
            },
            None => String::from("usuário"),
        }
    }

    pub async fn reply(&self, text: impl Into<String>) -> Result<Message, Error> {
        let message = self
            .bot
            .send_message(self.message.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(message)
    }

    pub async fn edit(&self, target: &Message, text: impl Into<String>) -> Result<(), Error> {
        self.bot
            .edit_message_text(target.chat.id, target.id, text)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }
}
