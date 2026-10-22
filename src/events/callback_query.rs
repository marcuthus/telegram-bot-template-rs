use teloxide::{prelude::*, types::CallbackQuery};
use tracing::info;

use crate::data::Error;

pub async fn run(bot: Bot, query: CallbackQuery) -> Result<(), Error> {
    info!(target: "client", "Callback query received: {:?}", query.data);

    bot.answer_callback_query(query.id).await?;

    Ok(())
}
