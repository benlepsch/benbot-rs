use crate::{Context, Error};
use poise::serenity_prelude as serenity;

use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::id::ChannelId;

use rand;
use rand::seq::IndexedRandom;

use std::env;

#[derive(Debug)]
pub struct Data {}


/* Commands */

// says hello
#[poise::command(slash_command, prefix_command)]
pub async fn say_hello(
	ctx: Context<'_>
) -> Result<(), Error> {
	ctx.say("i am poising").await?;
	Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn say(
	ctx: Context<'_>,
	#[description = "what to say"] text: String,
	#[description = "who is saying"] user: serenity::User,
) -> Result<(), Error> {
	let saying = format!("{} says: {}", user.name, text);
	
	ctx.say(saying).await?;
	Ok(())
}

// try a context menu command
#[poise::command(context_menu_command = "pin")]
pub async fn pin(
    ctx: Context<'_>,
    #[description = "test"] msg: serenity::Message,
) -> Result<(), Error> {
    // dbg!(ctx);
    let pins_channel_id = env::var("PINS_CHANNEL")
        .expect("pins channel missing from env")
        .parse::<u64>()
        .unwrap();

    let pins_channel = ChannelId::from(pins_channel_id);
    
    // println!("Link: {}", &msg.link());

    let memb = ctx.guild_id().expect("please")
        .to_partial_guild(&ctx.http()).await?
        .member(&ctx.http(), msg.author.id).await?;
    
    // dbg!(&msg);
    let nick = memb.nick.unwrap();
        // .unwrap_or_else(|| {msg.author.name});

    let body = format!("{}\n\n{}", &msg.content, &msg.link());

    // let footer = serenity::CreateEmbedFooter::new(format!("{} ", &msg.link()));

    let embed = CreateEmbed::new()
        .title(nick)
        .description(body);
        // .footer(footer);
    
    let builder = CreateMessage::new().embed(embed);

    pins_channel.send_message(&ctx.http(), builder).await?;
    ctx.say("ok").await?;
    Ok(())
}

// register command
#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}


const PAPRIKA_GIFS: &'static [&str] = &[
    "https://tenor.com/view/paprika-movie-anime-atsuko-chiba-satoshi-kon-gif-14134517",
    "https://tenor.com/view/paprika-gif-5430367",
    "https://tenor.com/view/paprika-gif-25394130",
    "https://tenor.com/view/paprika-paprika-anime-gif-10413276",
    "https://media.tenor.com/wxaaQuEOXQAAAAAC/anime-burger.gif",
];

// message handler for paprika
pub async fn event_handler(
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
