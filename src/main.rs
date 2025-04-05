use teloxide::{prelude::Requester, types::Message, Bot};
use rand::Rng;
use pretty_env_logger;

fn parse_dice_command(text: &str) -> Option<u32> {
    let text = text.trim().to_lowercase();
    if text.starts_with("roll d") {
        // this feels brittle, but it works for now
        text[6..].parse::<u32>().ok()
    } else {
        None
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            if let Some(sides) = parse_dice_command(text) {
                let result = rand::thread_rng().gen_range(1..=sides);
                if let Err(e) = bot.send_message(msg.chat.id, format!("🎲 Rolled a d{}: {}", sides, result)).await {
                    log::error!("Failed to send message: {}", e);
                }
            }
        }
        Ok(())
    })
    .await;
}
