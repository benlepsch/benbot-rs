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
	let token = env::var("BENBOT_TOKEN").expect("missing BENBOT_TOKEN from environment variables");
	let intents = serenity::GatewayIntents::non_privileged();

	let options = poise::FrameworkOptions {
		commands: vec![say_hello(), say(), register()],
		on_error: |error| Box::pin(on_error(error)),
		..Default::default()
	};

	let framework = poise::Framework::builder()
		.options(options)
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				println!("Logged in as {}", _ready.user.name);
				// poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data {})
			})
		})
		.build();
	
	let client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await;
	
	client.unwrap().start().await.unwrap();
}
