use serenity::prelude::*;
use std::collections::HashMap;

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}
