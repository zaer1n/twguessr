use crate::types::{Context, Error};

use poise::serenity_prelude as serenity;
use twmap::TwMap;
use vek::{Extent2, Vec2};

#[poise::command(slash_command, subcommands("kog", "ddnet", "file"))]
pub async fn render(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fn create_reply(mut map: TwMap) -> Result<poise::CreateReply, Error> {
    let image = twguessr::map_to_image(
        &mut map,
        1.0,
        Extent2::new(3840, 2160), // 4K
        Vec2::new(0, 0),
        true,
    )?;
    let image_bytes = twguessr::image_to_bytes(image, image::ImageFormat::Png)?;
    let attachment = serenity::CreateAttachment::bytes(image_bytes, "result.png");
    Ok(poise::CreateReply::default().attachment(attachment))
}

/// Renders a kog map.
#[poise::command(slash_command)]
pub async fn kog(
    ctx: Context<'_>,
    #[description = "Name of the map to render"] map: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let map = TwMap::parse_path(format!("./data/maps/kog/{map}.map"))?;
    ctx.send(create_reply(map)?).await?;
    Ok(())
}

/// Renders a ddnet map.
#[poise::command(slash_command)]
pub async fn ddnet(
    ctx: Context<'_>,
    #[description = "Name of the map to render"] map: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let map = TwMap::parse_path(format!("./data/maps/ddnet/{map}.map"))?;
    ctx.send(create_reply(map)?).await?;
    Ok(())
}

/// Renders a map file.
#[poise::command(slash_command)]
pub async fn file(
    ctx: Context<'_>,
    #[description = "Map file to be rendered"] map: serenity::Attachment,
) -> Result<(), Error> {
    ctx.defer().await?;
    let map = TwMap::parse(&map.download().await?)?;
    ctx.send(create_reply(map)?).await?;
    Ok(())
}
