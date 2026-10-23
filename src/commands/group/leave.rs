use teloxide::prelude::*;

use crate::{
    data::{Context, Error},
    utils::checks::require_chat_admin,
};

pub async fn run(ctx: &Context) -> Result<(), Error> {
    require_chat_admin(ctx).await?;

    ctx.reply("Até mais! 👋").await?;
    ctx.bot.leave_chat(ctx.message.chat.id).await?;

    Ok(())
}
