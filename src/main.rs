mod commands;

use poise::serenity_prelude as serenity;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, commands::Data, Error>;

/* Error handling */

async fn on_error(error: poise::FrameworkError<'_, commands::Data, Error>) {
	match error {
		poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
		poise::FrameworkError::Command {error, ctx, ..} => {
			println!("Error in command `{}`: {:?}", ctx.command().name, error);
		}

		error => {
			if let Err(e) = poise::builtins::on_error(error).await {
				println!("Error while handling error: {}", e);
			}
		}
	}
}

#[tokio::main]
async fn main() {
    let token = env::var("BENBOT_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents = serenity::GatewayIntents::non_privileged() 
		| serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(commands::Data { })
            })
        })
        .options(poise::FrameworkOptions {
			commands: vec![
                commands::say_hello(), 
                commands::say(), 
                commands::pin(),
                commands::register()
            ],
            event_handler: |ctx, event, framework, commands::Data {}| {
                Box::pin(commands::event_handler(ctx, event, framework, &commands::Data {}))
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
