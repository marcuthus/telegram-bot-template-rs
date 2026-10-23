use teloxide::{
    net::Download,
    prelude::*,
    types::{FileId, InputFile, Message, ReplyParameters},
};

use crate::{
    data::{Context, Error},
    utils::sticker::{StickerOptions, to_sticker},
};

pub async fn run(ctx: &Context, options: StickerOptions) -> Result<(), Error> {
    let Some(file_id) =
        find_image(&ctx.message).or_else(|| ctx.message.reply_to_message().and_then(find_image))
    else {
        ctx.reply("Envie uma foto com a legenda /sticker ou responda uma foto com /sticker.")
            .await?;
        return Ok(());
    };

    let file = ctx.bot.get_file(file_id).await?;
    let mut original = Vec::new();
    ctx.bot.download_file(&file.path, &mut original).await?;

    let webp = tokio::task::spawn_blocking(move || to_sticker(&original, options)).await??;

    ctx.bot
        .send_sticker(
            ctx.message.chat.id,
            InputFile::memory(webp).file_name("sticker.webp"),
        )
        .reply_parameters(ReplyParameters::new(ctx.message.id))
        .await?;

    Ok(())
}

fn find_image(message: &Message) -> Option<FileId> {
    if let Some(photo) = message.photo().and_then(|sizes| sizes.last()) {
        return Some(photo.file.id.clone());
    }

    message
        .document()
        .filter(|document| {
            document
                .mime_type
                .as_ref()
                .is_some_and(|mime| mime.essence_str().starts_with("image/"))
        })
        .map(|document| document.file.id.clone())
}
