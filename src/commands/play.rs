use crate::{types::Context, types::Error};

/// Starts a game round.
#[poise::command(slash_command)]
pub async fn play(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
