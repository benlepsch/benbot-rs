use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use std::env;

use tokio::time;
use chrono::{DateTime, Utc, Duration};


/* League status checker */

/*
- list of usernames/taglines to check for
- get riot api key from env
*/

async fn check_api(
    username: String, 
    tagline: String,
    ctx: Context<'_>,
) {
    // make api query
    // send a message with context if in game
}


#[poise::command(slash_command, prefix_command)]
pub async fn start_check(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let mut next = Utc::now();

    loop {
        next = next.checked_add_signed(Duration::seconds(60)).unwrap();
        let diff = (next - Utc::now()).to_std().unwrap();

        /* Check API */

        time::sleep(diff).await;
    }

    Ok(())
}