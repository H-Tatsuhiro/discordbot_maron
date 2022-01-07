use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("inu")]
#[description = "犬の絵文字"]
#[bucket = "emoji"]
pub async fn dog(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, ":dog:").await?;
    Ok(())
}
