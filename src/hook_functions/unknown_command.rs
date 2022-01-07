use serenity::prelude::*;
use serenity::{framework::standard::macros::hook, model::channel::Message};

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let notice = format!("'{}'コマンドが見つからないよ！", unknown_command_name);
    let _ = msg.reply(ctx, &notice).await;
}
