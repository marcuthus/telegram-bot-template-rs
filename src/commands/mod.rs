mod admin;
mod example;
mod group;
mod info;
mod media;

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
    utils::sticker::StickerOptions,
};

const CATEGORIES: [(&str, &[&str]); 5] = [
    ("ℹ️ Informações", &["ping", "id", "chat_id", "help", "menu"]),
    ("👥 Grupo", &["admins", "ban_member", "invite", "leave"]),
    ("🖼️ Mídia", &["sticker"]),
    ("🧪 Exemplos", &["sum"]),
    ("🔐 Administração", &["uptime"]),
];

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Comandos disponíveis:")]
pub enum Command {
    #[command(description = "Envia o ping do bot.")]
    Ping,
    #[command(
        description = "Mostra o seu ID ou o de quem for respondido/mencionado. Uso: /id [id]"
    )]
    Id(String),
    #[command(rename = "chat_id", description = "Mostra o ID do chat atual.")]
    ChatId,
    #[command(description = "Exibe esta ajuda.")]
    Help,
    #[command(description = "Exibe o menu de comandos por categoria.")]
    Menu,
    #[command(description = "Lista os administradores do grupo e seus IDs.")]
    Admins,
    #[command(
        rename = "ban_member",
        description = "Bane um membro do grupo (apenas administradores). Uso: responda uma mensagem com /ban_member"
    )]
    BanMember(String),
    #[command(description = "Gera o link de convite do grupo (apenas administradores).")]
    Invite,
    #[command(description = "Faz o bot sair do grupo (apenas administradores).")]
    Leave,
    #[command(
        description = "Cria um sticker a partir de uma foto. Uso: /sticker [crop: true|false|dented] [center: true|false]",
        parse_with = parse_sticker_options
    )]
    Sticker(StickerOptions),
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
            Self::Id(_) => "id",
            Self::ChatId => "chat_id",
            Self::Help => "help",
            Self::Menu => "menu",
            Self::Admins => "admins",
            Self::BanMember(_) => "ban_member",
            Self::Invite => "invite",
            Self::Leave => "leave",
            Self::Sticker(_) => "sticker",
            Self::Sum(..) => "sum",
            Self::Uptime => "uptime",
        }
    }

    pub fn menu(author_name: &str) -> String {
        let descriptions = Self::bot_commands();

        let sections = CATEGORIES.iter().map(|(label, names)| {
            let lines = names
                .iter()
                .filter_map(|name| {
                    descriptions
                        .iter()
                        .find(|command| command.command.trim_start_matches('/') == *name)
                        .map(|command| {
                            format!(
                                "• <code>/{}</code> - {}",
                                command.command.trim_start_matches('/'),
                                html::escape(&command.description)
                            )
                        })
                })
                .collect::<Vec<_>>();

            format!("<b>{label}</b>\n{}", lines.join("\n"))
        });

        format!(
            "<b>🤖 Menu</b>\n\nOlá {}, estes são os meus comandos:\n\n{}\n\n<b><i>👨‍💻 By Marcuth</i></b>",
            html::escape(author_name),
            sections.collect::<Vec<_>>().join("\n\n")
        )
    }
}

fn parse_sticker_options(input: String) -> Result<(StickerOptions,), ParseError> {
    let mut parts = input.split_whitespace();

    let options = StickerOptions::parse(parts.next(), parts.next())
        .map_err(|error| ParseError::IncorrectFormat(error.into()))?;

    if parts.next().is_some() {
        return Err(ParseError::Custom("argumentos demais".into()));
    }

    Ok((options,))
}

pub async fn handle_message(bot: Bot, message: Message, data: Arc<Data>) -> Result<(), Error> {
    let Some(text) = message
        .text()
        .or_else(|| message.caption())
        .filter(|text| text.starts_with('/'))
    else {
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
        Command::Id(argument) => info::id::run(ctx, argument).await,
        Command::ChatId => info::chat_id::run(ctx).await,
        Command::Help => info::help::run(ctx).await,
        Command::Menu => info::menu::run(ctx).await,
        Command::Admins => group::admins::run(ctx).await,
        Command::BanMember(argument) => group::ban_member::run(ctx, argument).await,
        Command::Invite => group::invite::run(ctx).await,
        Command::Leave => group::leave::run(ctx).await,
        Command::Sticker(options) => media::sticker::run(ctx, *options).await,
        Command::Sum(a, b) => example::sum::run(ctx, *a, *b).await,
        Command::Uptime => admin::uptime::run(ctx).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sticker::CropMode;

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

    #[test]
    fn parses_id_with_and_without_an_argument() {
        assert!(
            matches!(Command::parse("/id", "bot"), Ok(Command::Id(argument)) if argument.is_empty())
        );
        assert!(
            matches!(Command::parse("/id 42", "bot"), Ok(Command::Id(argument)) if argument == "42")
        );
    }

    #[test]
    fn parses_commands_with_underscores() {
        assert!(matches!(
            Command::parse("/chat_id", "bot"),
            Ok(Command::ChatId)
        ));
        assert!(matches!(
            Command::parse("/ban_member 42", "bot"),
            Ok(Command::BanMember(argument)) if argument == "42"
        ));
    }

    #[test]
    fn parses_sticker_options() {
        assert!(matches!(
            Command::parse("/sticker", "bot"),
            Ok(Command::Sticker(options)) if options == StickerOptions::default()
        ));
        assert!(matches!(
            Command::parse("/sticker dented false", "bot"),
            Ok(Command::Sticker(options)) if options.crop == CropMode::Dented && !options.center
        ));
        assert!(Command::parse("/sticker maybe", "bot").is_err());
        assert!(Command::parse("/sticker true true extra", "bot").is_err());
    }

    #[test]
    fn every_command_is_listed_in_a_menu_category() {
        let listed = CATEGORIES
            .iter()
            .flat_map(|(_, names)| names.iter().copied())
            .collect::<Vec<_>>();

        for command in Command::bot_commands() {
            assert!(
                listed.contains(&command.command.trim_start_matches('/')),
                "{}",
                command.command
            );
        }

        assert_eq!(listed.len(), Command::bot_commands().len());
    }

    #[test]
    fn menu_groups_commands_by_category() {
        let menu = Command::menu("Ana");

        assert!(menu.contains("Olá Ana"));
        assert!(menu.find("Informações").unwrap() < menu.find("/ping").unwrap());
        assert!(menu.find("/ping").unwrap() < menu.find("Grupo").unwrap());
        assert!(menu.find("Grupo").unwrap() < menu.find("/ban_member").unwrap());
    }
}
