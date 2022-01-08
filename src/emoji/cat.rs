use serenity::framework::standard::buckets::RevertBucket;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("kitty", "neko")]
#[bucket = "emoji"]
#[required_permissions("ADMINISTRATOR")]
pub async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, ":cat:").await?;
    Err(RevertBucket.into())
}
