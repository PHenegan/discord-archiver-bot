mod backup;

use poise::{Framework, FrameworkOptions};
use poise::samples::register_globally;
use serenity::all::{ClientBuilder, GatewayIntents};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn backup(ctx: Context<'_>) -> Result<(), Error> {
    let result = backup::backup(ctx, 1000).await;
    let message = match result {
        Ok(_) => "Command ran successfully".to_string(),
        Err(err) => format!("Command failed: {err:#?}")
    };
    ctx.say(message).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Should have Discord Bot token in environment variable");
    let intents = GatewayIntents::non_privileged();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![backup()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap()
        .start().await
        .unwrap();
}
