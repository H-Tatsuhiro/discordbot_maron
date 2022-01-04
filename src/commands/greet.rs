use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "あいさつ"]
pub async fn greet(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, format!("{} こんにちわ！", msg.author.mention()))
        .await?;
    Ok(())
}
