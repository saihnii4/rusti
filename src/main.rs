#![allow(deprecated)]

mod cmd;

use cmd::*;
use std::{env, sync::{Arc, RwLock}};

use serenity::{
    all::{
        standard::{
            macros::{command, group, hook},
            CommandResult, Configuration,
        }, ClientBuilder, Message, Ready, StandardFramework,
    },
    async_trait,
    prelude::*,
};

use std::collections::HashMap;

struct Handler;

struct SnipeBucket;

impl TypeMapKey for SnipeBucket {
    type Value = Arc<RwLock<HashMap<u32, Message>>>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if ctx.cache.current_user().id == msg.author.id {
            return;
        }

        println!("{} said \"{}\"", msg.author.name, msg.content);
    }
    
    async fn message_delete(&self, ctx: Context, msg: Message) {
        if ctx.cache.current_user().bot {
            ()
        }

        let mut wacc = ctx.data.write().await;
        let mut snipe_acc =  wacc.get::<SnipeBucket>().expect("owo").clone();
        snipe_acc
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Logged in as {:?}", ctx.cache.current_user().name);
    }
}

#[hook]
async fn before(_: &Context, msg: &Message, cmd: &str) -> bool {
    println!("{} executed command \"{}\"", msg.author.id, cmd);
    true
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("dotenv not able to be found");
    let tok = env::var("TOKEN").expect("token not found in dotenv");
    let config = Configuration::new()
        .prefix("~")
        .allow_dm(false)
        .ignore_bots(true);
    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .before(before);
    framework.configure(config);
    let mut cl = ClientBuilder::new(tok, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("client build failed");
    {
        let mut wacc = cl.data.write().await;
        wacc.insert::<SnipeBucket>(Arc::new(RwLock::new(HashMap::default())));
    }
    if let Err(err) = cl.start().await {
        println!("client start failed: {}", err);
    }
}
