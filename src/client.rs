use std::{sync::Arc, time::Instant};

use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::info;

use crate::{
    commands::{self, Command},
    data::Data,
    events,
    helpers::config::Config,
};

pub async fn start(config: Config) -> anyhow::Result<()> {
    info!(target: "client", "Starting...");

    let bot = Bot::new(config.token.clone());
    let me = bot.get_me().await?;

    bot.set_my_commands(Command::bot_commands()).await?;
    info!(target: "client", "All commands synchronized with Telegram!");

    let data = Arc::new(Data {
        config: Arc::new(config),
        bot_username: me.username().to_owned(),
        started_at: Instant::now(),
    });

    events::ready::run(&me);

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(commands::handle_message))
        .branch(Update::filter_callback_query().endpoint(events::callback_query::run));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![data])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    info!(target: "client", "Successfully stopped!");

    Ok(())
}
