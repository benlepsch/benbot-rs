use poise::serenity_prelude as serenity;
use std::env;

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

/* Event handling? */
struct Handler {
	options: poise::FrameworkOptions<Data, Error>,
}

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
	async fn message(&self, ctx: serenity::Context, new_message: serenity::Message) {
		if new_message.content == "hello benbot" {
            if let Err(why) = new_message.channel_id.say(&ctx.http, "i am rusting").await {
                println!("Error sending message: {why:?}");
            }
        }
 
	}
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

#[tokio::main]
async fn main() -> Result<(), Error> {
	let token = env::var("BENBOT_TOKEN").expect("missing BENBOT_TOKEN from environment variables");
	let intents = serenity::GatewayIntents::non_privileged();

	let options = poise::FrameworkOptions {
		commands: vec![say_hello(), say(), register()],
		on_error: |error| Box::pin(on_error(error)),
		..Default::default()
	};

	let mut handler = Handler {
		options: options,
	};
	poise::set_qualified_names(&mut handler.options.commands);

	let handler = std::sync::Arc::new(handler);
	let mut client = serenity::Client::builder(token, intents)
		.event_handler_arc(handler.clone())
		.await?;

	client.start().await?;

	Ok(())
}
