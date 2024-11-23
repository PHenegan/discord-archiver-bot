use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;
use std::io::Write;

use serenity::all::{GetMessages, Message};

use crate::{Context, Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DiscordBackupError;

impl Display for DiscordBackupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error backing up Discord Channel")
    }
}

impl std::error::Error for DiscordBackupError {}

const MAX_MESSAGES: u8 = 100;

pub async fn backup(ctx: Context<'_>, count: usize) -> Result<(), Error> {
    println!("Received backup request");
    let channel = ctx.guild_channel().await
        .ok_or(Box::new(DiscordBackupError))?;

    ctx.say(format!(
        "Received command for channel #{}, counting messages",
        channel.name()
    )).await?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(format!("./{}.txt", channel.name()))?;

    let mut message_builder = GetMessages::new().limit(MAX_MESSAGES);
    let mut total_count = 0;

    let mut finished = false;
    while !finished && total_count < count {
        let messages = channel.messages(ctx, message_builder).await?;
        total_count += messages.len();
        finished = messages.len() != (MAX_MESSAGES as usize);
        let text_messages: String = messages.iter()
            .map(|message| format_message(message))
            .fold("".to_string(), |result, message| result + "\n" + message.as_str());
        file.write(text_messages.into_bytes().as_slice())?;

        if messages.len() > 0 {
            message_builder = message_builder.before(messages.last().unwrap().id);
        }
        println!("Processed {} total messages", total_count);
    }
    ctx.say(format!("got {} messages!", total_count)).await?;
    Ok(())
}

fn format_message(message: &Message) -> String {
    let timestamp = message.timestamp.format("");
    let user = message.author.name.clone();
    let message = message.content.clone();
    format!("({timestamp})[{user}] {message}")
}
