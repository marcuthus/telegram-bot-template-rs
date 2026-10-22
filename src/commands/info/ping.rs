use std::time::Instant;

use teloxide::utils::html;

use crate::data::{Context, Error};

pub async fn run(ctx: &Context) -> Result<(), Error> {
    let started = Instant::now();
    let sent = ctx.reply("Calculando ping...").await?;
    let message_latency = started.elapsed().as_millis();

    let reply = format!(
        "<b>🏓 Pong!</b>\n\nOlá {}, meu ping está em <code>{message_latency}ms</code>\n\n<b><i>👨‍💻 By Marcuth</i></b>",
        html::escape(&ctx.author_mention())
    );

    ctx.edit(&sent, reply).await
}
