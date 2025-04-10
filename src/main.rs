//! poise::Framework handles client creation and event handling for you. Alternatively, you can
//! do that yourself and merely forward the events you receive to poise. This example shows how.
//!
//! Note: this example configures no designated prefix. Mention the bot as a prefix instead. For
//! that to work, please adjust the bot ID below to your bot, for the mention parsing to work.

use poise::serenity_prelude as serenity;

type Error = serenity::Error;

/* static paprika_gifs = vec![
	"https://tenor.com/view/paprika-movie-anime-atsuko-chiba-satoshi-kon-gif-14134517",
    "https://tenor.com/view/paprika-gif-5430367",
    "https://tenor.com/view/paprika-gif-25394130",
	"https://tenor.com/view/paprika-paprika-anime-gif-10413276",
    "https://media.tenor.com/wxaaQuEOXQAAAAAC/anime-burger.gif",
]; */

#[poise::command(prefix_command)]
async fn ping(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

struct Handler {
    options: poise::FrameworkOptions<(), Error>,
    shard_manager: std::sync::Mutex<Option<std::sync::Arc<serenity::ShardManager>>>,
}

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, mut new_message: serenity::Message) {
		// println!("message received: {}", new_message.content);
 		let bot_id = serenity::UserId::new(493938037189902358);

		if new_message.author.id == bot_id {
			return
		}
		
		new_message.content.make_ascii_lowercase();
		if new_message.content.contains("paprika") {
			if let Err(why) = new_message.channel_id.say(&ctx.http, "paprika").await {
				println!("Error sending message: {why:?}");
			}
		}

		// FrameworkContext contains all data that poise::Framework usually manages
        let shard_manager = (*self.shard_manager.lock().unwrap()).clone().unwrap();
        let framework_data = poise::FrameworkContext {
            bot_id: bot_id,
            options: &self.options,
            user_data: &(),
            shard_manager: &shard_manager,
        };

 	

 		let event = serenity::FullEvent::Message { new_message };
        poise::dispatch_event(framework_data, &ctx, event).await;
    }

    // For slash commands or edit tracking to work, forward interaction_create and message_update
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = std::env::var("BENBOT_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
		| serenity::GatewayIntents::MESSAGE_CONTENT;
    let mut handler = Handler {
        options: poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        },
        shard_manager: std::sync::Mutex::new(None),
    };
    poise::set_qualified_names(&mut handler.options.commands); // some setup

    let handler = std::sync::Arc::new(handler);
    let mut client = serenity::Client::builder(token, intents)
        .event_handler_arc(handler.clone())
        .await?;

    *handler.shard_manager.lock().unwrap() = Some(client.shard_manager.clone());
    client.start().await?;

    Ok(())
}
