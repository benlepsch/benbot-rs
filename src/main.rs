use poise::serenity_prelude as serenity;
use std::env;

use rand;
use rand::seq::IndexedRandom;
struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


/* Commands */

// says hello
#[poise::command(slash_command, prefix_command)]
async fn say_hello(
	ctx: Context<'_>
) -> Result<(), Error> {
	ctx.say("i am poising").await?;
	Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn say(
	ctx: Context<'_>,
	#[description = "what to say"] text: String,
	#[description = "who is saying"] user: serenity::User,
) -> Result<(), Error> {
	let saying = format!("{} says: {}", user.name, text);
	
	ctx.say(saying).await?;
	Ok(())
}

// register command
#[poise::command(prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}


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

const PAPRIKA_GIFS: &'static [&str] = &[
    "https://tenor.com/view/paprika-movie-anime-atsuko-chiba-satoshi-kon-gif-14134517",
    "https://tenor.com/view/paprika-gif-5430367",
    "https://tenor.com/view/paprika-gif-25394130",
    "https://tenor.com/view/paprika-paprika-anime-gif-10413276",
    "https://media.tenor.com/wxaaQuEOXQAAAAAC/anime-burger.gif",
];

#[tokio::main]
async fn main() {
    let token = env::var("BENBOT_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents = serenity::GatewayIntents::non_privileged() 
		| serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data { })
            })
        })
        .options(poise::FrameworkOptions {
			commands: vec![say_hello(), say(), register()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
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

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase().contains("paprika")
                && new_message.author.id != ctx.cache.current_user().id {
                let chosen_gif = {
                    let mut rng = rand::rng();
                    PAPRIKA_GIFS.choose(&mut rng).expect("should pick a gif")
                };

                new_message.reply(ctx, *chosen_gif).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
