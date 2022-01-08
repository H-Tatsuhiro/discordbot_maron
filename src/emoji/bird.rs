use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[aliases("tori")]
#[description = "鳥の絵文字"]
pub async fn bird(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let content = if args.is_empty() {
        ":bird:".to_string()
    } else {
        format!(":bird: '{}'という動物が見つからないよ！", args.rest())
    };

    msg.channel_id.say(&ctx.http, content).await?;
    Ok(())
}
