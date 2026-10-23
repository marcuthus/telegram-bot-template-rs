use teloxide::utils::html;

use crate::data::{Context, Error};

pub async fn run(ctx: &Context) -> Result<(), Error> {
    let title = ctx
        .message
        .chat
        .title()
        .map(|title| format!(" ({})", html::escape(title)))
        .unwrap_or_default();

    ctx.reply(format!(
        "O ID deste chat{title} é <code>{}</code>",
        ctx.message.chat.id
    ))
    .await?;

    Ok(())
}
