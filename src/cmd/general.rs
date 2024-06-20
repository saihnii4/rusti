use serenity::all::{
    standard::{
        macros::{command, group},
        CommandResult,
    },
    Context, Message,
};

use crate::SnipeBucket;

#[group]
#[commands(ping)]
pub struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong!").await.expect("byebye");
    Ok(())
}

#[command]
async fn uptime(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO:
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
async fn snipe(ctx: &Context, msg: &Message) -> CommandResult {
    let lock = {
        let data_access = ctx.data.read().await;
        data_access.get::<SnipeBucket>().expect("hai").clone()
    };
    let last_snipe = {
        let map = lock.read().await;
        map.get(&msg.channel_id.into())
    };
    msg.reply(
        ctx,
    )
    .await
    .unwrap();
    Ok(())
}
