use serenity::all::{
    standard::{
        macros::{command, group},
        CommandResult,
    },
    Context, Message,
};

#[group]
struct Math;

#[command]
async fn add(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}
