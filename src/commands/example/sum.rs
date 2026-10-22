use crate::data::{Context, Error};

pub async fn run(ctx: &Context, a: f64, b: f64) -> Result<(), Error> {
    ctx.reply(format!("<code>{a} + {b} = {}</code>", a + b))
        .await?;

    Ok(())
}
