use teloxide::{prelude::*, types::UserId, utils::html};

use crate::{
    data::{Context, Error, mention_of},
    utils::checks::require_chat_admin,
};

pub async fn run(ctx: &Context, argument: &str) -> Result<(), Error> {
    require_chat_admin(ctx).await?;

    let target = ctx
        .target_user()
        .map(|user| (user.id, mention_of(&user)))
        .or_else(|| {
            argument
                .trim()
                .parse::<u64>()
                .ok()
                .map(|id| (UserId(id), id.to_string()))
        });

    let Some((user_id, name)) = target else {
        ctx.reply(
            "Responda a mensagem de quem deve ser banido ou informe o ID. Uso: /ban_member [id]",
        )
        .await?;
        return Ok(());
    };

    if ctx.author_id() == Some(user_id.0) {
        ctx.reply("Você não pode banir a si mesmo.").await?;
        return Ok(());
    }

    let reply = match ctx.bot.ban_chat_member(ctx.message.chat.id, user_id).await {
        Ok(_) => format!("<b>{}</b> foi removido do grupo.", html::escape(&name)),
        Err(error) => format!(
            "Não foi possível banir <code>{user_id}</code>. O bot precisa ser administrador com permissão de banir. ({})",
            html::escape(&error.to_string())
        ),
    };

    ctx.reply(reply).await?;

    Ok(())
}
