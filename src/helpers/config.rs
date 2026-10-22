use std::env;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("a variável de ambiente `{0}` não foi definida")]
    Missing(&'static str),
}

#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub admin_id: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            token: required("TELEGRAM_BOT_TOKEN")?,
            admin_id: required("TELEGRAM_ADMIN_ID")?,
        })
    }
}

fn required(key: &'static str) -> Result<String, ConfigError> {
    match env::var(key) {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        _ => Err(ConfigError::Missing(key)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_fails_when_variable_is_missing() {
        let error = required("TELEGRAM_BOT_RS_UNSET_VARIABLE").unwrap_err();

        assert_eq!(
            error.to_string(),
            "a variável de ambiente `TELEGRAM_BOT_RS_UNSET_VARIABLE` não foi definida"
        );
    }
}
