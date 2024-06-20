#![allow(deprecated)]

mod cmd;

use cmd::*;
use std::{env, ops::Deref, sync::Arc};

use serenity::{
    all::{
        standard::{
            macros::{command, group, hook},
            CommandResult, Configuration,
        },
        ChannelId, ClientBuilder, GuildId, Message, MessageId, Ready, StandardFramework,
    },
    async_trait,
    prelude::*,
};

use std::collections::HashMap;

struct Handler;

struct SnipeBucket;

// intuitively the defunct message should get deallocated off the heap once its replaced by a
// fresher message.
impl TypeMapKey for SnipeBucket {
    type Value = Arc<RwLock<HashMap<u64, Message>>>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if ctx.cache.current_user().id == msg.author.id {
            return;
        }

        println!("{} said \"{}\"", msg.author.name, msg.content);
    }

    async fn message_delete(
        &self,
        ctx: Context,
        ch: ChannelId,
        msg: MessageId,
        _server: Option<GuildId>,
    ) {
        if ctx.cache.current_user().bot {
            return;
        }

        let snipe_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<SnipeBucket>().expect("bruh").clone()
        };

        {
            let mut bucket = snipe_lock.write().await;
            bucket
                .entry(ch.into())
                .or_insert(ctx.cache.message(ch, msg).unwrap().deref().clone());
            // perhaps not too costly? idk
        }
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
