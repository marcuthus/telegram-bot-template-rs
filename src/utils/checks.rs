use teloxide::prelude::*;

use crate::{
    data::{Context, Error},
    error::CommandError,
};

pub fn is_bot_owner_id(id: u64, admin_id: &str) -> bool {
    id.to_string() == admin_id
}

pub fn require_group(ctx: &Context) -> Result<(), Error> {
    if ctx.message.chat.is_group() || ctx.message.chat.is_supergroup() {
        Ok(())
    } else {
        Err(CommandError::NotInGroup.into())
    }
}

pub async fn require_chat_admin(ctx: &Context) -> Result<(), Error> {
    require_group(ctx)?;

    let Some(author) = ctx.message.from.as_ref() else {
        return Err(CommandError::PermissionDenied.into());
    };

    if is_bot_owner_id(author.id.0, &ctx.data.config.admin_id) {
        return Ok(());
    }

    let member = ctx
        .bot
        .get_chat_member(ctx.message.chat.id, author.id)
        .await?;

    if member.is_privileged() {
        Ok(())
    } else {
        Err(CommandError::PermissionDenied.into())
    }
}

pub fn require_bot_owner(ctx: &Context) -> Result<(), Error> {
    let is_owner = ctx
        .author_id()
        .is_some_and(|id| is_bot_owner_id(id, &ctx.data.config.admin_id));

    if is_owner {
        Ok(())
    } else {
        Err(CommandError::PermissionDenied.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_only_the_configured_admin() {
        assert!(is_bot_owner_id(123, "123"));
        assert!(!is_bot_owner_id(456, "123"));
        assert!(!is_bot_owner_id(123, ""));
    }
}
