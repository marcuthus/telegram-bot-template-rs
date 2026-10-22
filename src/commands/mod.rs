mod admin;
mod example;
mod info;

use std::sync::Arc;

use teloxide::{
    prelude::*,
    types::Message,
    utils::{
        command::{BotCommands, ParseError},
        html,
    },
};
use tracing::info;

use crate::{
    data::{Context, Data, Error},
    error,
};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Comandos disponíveis:")]
pub enum Command {
    #[command(description = "Envia o ping do bot.")]
    Ping,
    #[command(description = "Mostra o seu ID de usuário.")]
    Id,
    #[command(description = "Exibe esta ajuda.")]
    Help,
    #[command(
        description = "Soma dois números. Uso: /sum <a> <b>",
        parse_with = "split"
    )]
    Sum(f64, f64),
    #[command(description = "Mostra há quanto tempo o bot está online (apenas dono).")]
    Uptime,
}

impl Command {
    fn name(&self) -> &'static str {
        match self {
            Self::Ping => "ping",
            Self::Id => "id",
            Self::Help => "help",
            Self::Sum(..) => "sum",
            Self::Uptime => "uptime",
        }
    }
}

pub async fn handle_message(bot: Bot, message: Message, data: Arc<Data>) -> Result<(), Error> {
    let Some(text) = message.text().filter(|text| text.starts_with('/')) else {
        return Ok(());
    };

    let parsed = Command::parse(text, &data.bot_username);
    let ctx = Context::new(bot, message, data);

    let command = match parsed {
        Ok(command) => command,
        Err(ParseError::WrongBotName(_)) => return Ok(()),
        Err(ParseError::UnknownCommand(name)) => {
            let reply = format!("O comando <code>{}</code> não existe!", html::escape(&name));
            ctx.reply(reply).await?;
            return Ok(());
        }
        Err(parse_error) => {
            let reply = format!(
                "<code>Erro de argumentos:</code>\n\n{}",
                html::escape(&parse_error.to_string())
            );
            ctx.reply(reply).await?;
            return Ok(());
        }
    };

    info!(target: "client", "Running the command '{}'.", command.name());

    if let Err(error) = run(&ctx, &command).await {
        error::handle(&ctx, command.name(), error).await;
    }

    Ok(())
}

async fn run(ctx: &Context, command: &Command) -> Result<(), Error> {
    match command {
        Command::Ping => info::ping::run(ctx).await,
        Command::Id => info::id::run(ctx).await,
        Command::Help => info::help::run(ctx).await,
        Command::Sum(a, b) => example::sum::run(ctx, *a, *b).await,
        Command::Uptime => admin::uptime::run(ctx).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sum_with_two_numbers() {
        let command = Command::parse("/sum 2 3.5", "bot").unwrap();

        assert!(matches!(command, Command::Sum(a, b) if a == 2.0 && b == 3.5));
    }

    #[test]
    fn rejects_sum_with_invalid_arguments() {
        assert!(Command::parse("/sum x 1", "bot").is_err());
        assert!(Command::parse("/sum 1", "bot").is_err());
    }

    #[test]
    fn ignores_commands_addressed_to_other_bots() {
        assert!(matches!(
            Command::parse("/ping@other_bot", "bot"),
            Err(ParseError::WrongBotName(_))
        ));
    }
}
