use poise::serenity_prelude as serenity;
use std::env;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// says hello
#[poise::command(slash_command, prefix_command)]
async fn say_hello(
	ctx: Context<'_>
) -> Result<(), Error> {
	ctx.say("i am poising").await?;
	Ok(())
}

#[tokio::main]
async fn main() {
	let token = env::var("BENBOT_TOKEN").expect("missing BENBOT_TOKEN from environment variables");
	let intents = serenity::GatewayIntents::non_privileged();

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![say_hello()],
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				println!("Logged in as {}", _ready.user.name);
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data {})
			})
		})
		.build();
	
	let client = serenity::ClientBuilder::new(token, intents)
		.framework(framework)
		.await;
	
	client.unwrap().start().await.unwrap();
}
