mod commands;
mod leaguer;

use poise::serenity_prelude as serenity;
use std::env;
use sqlx::{Pool, Sqlite, sqlite, Executor};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Data {
    db: Pool<Sqlite>,
    api_key: String,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/* Error handling */

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
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
    let opt = sqlite::SqliteConnectOptions::new()
        .filename("test.db")
        .create_if_missing(true);

    let connection: Pool<Sqlite> = sqlite::SqlitePool::connect_with(opt).await.unwrap();
    
    connection.execute("
        CREATE TABLE IF NOT EXISTS leaguers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            tag_line TEXT,
            puuid TEXT,
            in_game BOOL
        )
    ").await.unwrap();

    let bot_data: Data = Data {
        db: connection,
        api_key: "RGAPI-a5a10c2d-92ef-40a9-8cc5-2a26e3921e07".to_string(),
    };

    let token = env::var("BENBOT_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents = serenity::GatewayIntents::non_privileged() 
		| serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(bot_data)
            })
        })
        .options(poise::FrameworkOptions {
			commands: vec![
                commands::ip(),
                commands::say_hello(), 
                commands::say(), 
                commands::monkey(),
                commands::src(),
                commands::pin(),
                commands::register(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(commands::event_handler(ctx, event, framework, data))
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
