use std::{collections::HashSet, fs::File, io::BufReader, path::PathBuf};

use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use serenity::model::channel::MessageActivity;
use serenity::model::user::User;

use discordbot_maron::commands::greet;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

use discordbot_maron::commands::{greet::*};

#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(greet)]
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

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

fn get_token(file_name: &PathBuf) -> Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}


#[tokio::main]
async fn main() {
    let mut path = PathBuf::new();
    path.push("config.json");
    let token = get_token(&path).expect("Failed to read the token.");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client.");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_token() {
        let mut path = PathBuf::new();
        path.push("config.json");
        let token = get_token(&path);
        match token {
            Ok(x) => {
                println!("{}", x);
                assert!(true);
            },
            Err(e) => assert!(false),
        }
    }
}
