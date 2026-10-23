use teloxide::utils::html;

use crate::data::{Context, Error, mention_of};

pub async fn run(ctx: &Context, argument: &str) -> Result<(), Error> {
    if let Some(user) = ctx.target_user() {
        ctx.reply(format!(
            "O ID de <b>{}</b> é <code>{}</code>",
            html::escape(&mention_of(&user)),
            user.id
        ))
        .await?;
        return Ok(());
    }

    if let Ok(id) = argument.trim().parse::<u64>() {
        ctx.reply(format!("O ID informado é <code>{id}</code>"))
            .await?;
        return Ok(());
    }

    let Some(id) = ctx.author_id() else {
        return Ok(());
    };

    ctx.reply(format!("O seu ID de usuário é <code>{id}</code>"))
        .await?;

    Ok(())
}
