use serenity::framework::standard::buckets::RevertBucket;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("kitty", "neko")]
#[description = "猫の絵文字"]
#[bucket = "emoji"]
pub async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if role
                .to_role_cached(&ctx.cache)
                .await
                .map_or(false, |r| r.name == "Event Staff")
            {
                msg.reply(&ctx, ":cat:").await?;
            }
        }
    }
    Err(RevertBucket.into())
}
