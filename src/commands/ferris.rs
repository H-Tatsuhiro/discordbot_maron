use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[command]
#[description = "Maronと友達のferrisからのあいさつ"]
pub async fn ferris(ctx: &Context, msg: &Message) -> CommandResult {
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("はじめまして！")
                .embed(|e| {
                    e.title("ごあいさつ")
                        .description("はじめましてのご挨拶")
                        .image("attachment://ferris.jpeg")
                        .fields(vec![("名前", "ferris", true), ("出身", "Rust", true)])
                        .field("好きなもの", "メモリ", false)
                        .footer(|f| f.text("自画像"))
                        .timestamp("20220105T231000+0900")
                })
                .add_file("./img/ferris.jpeg")
        })
        .await;

    if let Err(e) = msg {
        println!("Error sending message: {:?}", e);
    }
    Ok(())
}
