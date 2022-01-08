use crate::emoji::cat::CAT_COMMAND;
use crate::emoji::dog::DOG_COMMAND;
use serenity::framework::standard::macros::group;

#[group]
#[prefixes("emoji", "em")]
#[description = "絵文字メッセージのコマンドグループ"]
#[summary = "色々な絵文字を使えるよ"]
#[default_command(dog)]
#[commands(cat)]
pub struct Emoji;
