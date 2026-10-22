use teloxide::utils::{command::BotCommands, html};

use crate::{
    commands::Command,
    data::{Context, Error},
};

pub async fn run(ctx: &Context) -> Result<(), Error> {
    let descriptions = Command::descriptions().to_string();

    ctx.reply(html::escape(&descriptions)).await?;

    Ok(())
}
