use teloxide::{prelude::*, utils::html};

use crate::{
    data::{Context, Error},
    utils::checks::require_group,
};

pub async fn run(ctx: &Context) -> Result<(), Error> {
    require_group(ctx)?;

    let administrators = ctx.bot.get_chat_administrators(ctx.message.chat.id).await?;

    let lines = administrators
        .iter()
        .enumerate()
        .map(|(index, member)| {
            let crown = if member.is_owner() { "👑 " } else { "" };
            let username = member
                .user
                .username
                .as_ref()
                .map(|username| format!(" (@{username})"))
                .unwrap_or_default();

            format!(
                "{}. {crown}{}{} - <code>{}</code>",
                index + 1,
                html::escape(&member.user.first_name),
                html::escape(&username),
                member.user.id
            )
        })
        .collect::<Vec<_>>();

    ctx.reply(format!(
        "<b>👥 Administradores ({}):</b>\n\n{}",
        lines.len(),
        lines.join("\n")
    ))
    .await?;

    Ok(())
}
