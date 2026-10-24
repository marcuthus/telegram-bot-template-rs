use teloxide::{prelude::*, types::UserId, utils::html};

use crate::{
    data::{Context, Error, mention_of},
    utils::checks::{is_bot_owner_id, require_chat_admin},
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

    // O bot bane com as próprias permissões: sem checar o alvo, um administrador comum poderia
    // remover outros administradores (ou o dono) que o bot tenha permissão de banir.
    let bot_id = ctx.bot.get_me().await?.id;

    if user_id == bot_id {
        ctx.reply("Não posso banir a mim mesmo.").await?;
        return Ok(());
    }

    let target_is_staff = match ctx.bot.get_chat_member(ctx.message.chat.id, user_id).await {
        Ok(member) => member.is_privileged(),
        Err(_) => false,
    };

    let author_is_creator = match ctx.author_id() {
        Some(author_id) => {
            is_bot_owner_id(author_id, &ctx.data.config.admin_id)
                || ctx
                    .bot
                    .get_chat_member(ctx.message.chat.id, UserId(author_id))
                    .await
                    .map(|member| member.is_owner())
                    .unwrap_or(false)
        }
        None => false,
    };

    if target_is_staff && !author_is_creator {
        ctx.reply("Apenas o criador do grupo pode banir outro administrador.")
            .await?;
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
