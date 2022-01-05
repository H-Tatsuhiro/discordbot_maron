use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
#[description = "怒られてシャード番号を返す"]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = match msg.channel_id.to_channel(&ctx).await {
        Ok(channel) => channel,
        Err(e) => {
            println!("Error getting channel: {:?}", e);
            return Ok(());
        }
    };

    let response = MessageBuilder::new()
        .push_bold_safe(&msg.author.name)
        .push("さん！")
        .mention(&channel)
        .push("に居ないで早く寝なさい！\n")
        .push(format!("ちなみに今は{}番シャードにいるよ！", ctx.shard_id))
        .build();

    if let Err(e) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", e);
    }

    Ok(())
}
