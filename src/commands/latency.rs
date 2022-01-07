use crate::client::shard_manage::ShardManagerContainer;
use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "シャードの遅延を表示"]
pub async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            msg.reply(ctx, "ShardManagerが見つからないよ！").await?;
            return Ok(());
        }
    };
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            msg.reply(ctx, "Shardが見つからないよ！").await?;
            return Ok(());
        }
    };

    let late = match runner.latency {
        Some(x) => x,
        None => std::time::Duration::from_millis(0),
    };

    msg.reply(ctx, &format!("Shardの遅延は {:?} だよ！", &late))
        .await?;
    Ok(())
}
