use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // set handler for `message` event
    // called whenever a new message is received

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "hello benbot" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "i am rusting").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("BENBOT_TOKEN").expect("Expected a token in the environment");

    // intents decide what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
