use teloxide::prelude::*;

use crate::{
    data::{Context, Error},
    utils::checks::require_chat_admin,
};

pub async fn run(ctx: &Context) -> Result<(), Error> {
    require_chat_admin(ctx).await?;

    let reply = match ctx.bot.export_chat_invite_link(ctx.message.chat.id).await {
        Ok(link) => format!("Link de convite:\n{link}"),
        Err(_) => String::from(
            "Não foi possível gerar o link. O bot precisa ser administrador com permissão de convidar.",
        ),
    };

    ctx.reply(reply).await?;

    Ok(())
}
