use std::{collections::HashSet, env};

use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!messageme" {
            let dm = msg.author.dm(&ctx, |m| m.content("はじめまして！")).await;

            if let Err(e) = dm {
                println!("Error when direct messaging user: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

use discordbot_maron::commands::{about::*, ferris::*, greet::*, latency::*, ping::*};

#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(greet, about, latency, ping, ferris)]
struct General;

#[help]
#[individual_command_tip = "ヘルプコマンド"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client.");

    if let Err(e) = client.start_shards(1).await {
        println!("Client error: {:?}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_env() {
        let secret = env::var("DISCORD_TOKEN");
        match secret {
            Ok(x) => {
                println!("{}", x);
            }
            Err(_) => {
                panic!();
            }
        }
    }
}
