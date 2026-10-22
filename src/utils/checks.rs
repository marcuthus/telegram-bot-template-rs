use crate::{
    data::{Context, Error},
    error::CommandError,
};

pub fn is_bot_owner_id(id: u64, admin_id: &str) -> bool {
    id.to_string() == admin_id
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
