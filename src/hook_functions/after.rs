use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::hook, CommandResult},
    model::channel::Message,
};

#[hook]
pub async fn after(
    _ctx: &Context,
    _msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(e) => println!("Command '{}' returned error {:?}", command_name, e),
    }
}
