use teloxide::prelude::*;
use teloxide::types::{ InlineKeyboardButton, InlineKeyboardMarkup, ParseMode };
use base64::Engine;
use std::env;
use log::LevelFilter;
use reqwest;

const URL: &str = "https://example.com";

async fn start(message: Message, bot: Bot) -> ResponseResult<()> {
    let mut params = Vec::new();
    if let Some(text) = message.text() {
        let args: Vec<&str> = text.splitn(2, ' ').collect();
        let data_str = if args.len() > 1 { args[1] } else { "" };

        let decoded_data = base64::engine::general_purpose::URL_SAFE
            .decode(data_str)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok());

        if let Some(decoded_data) = decoded_data {
            let ref_index = decoded_data.find("r=");
            let query_index = decoded_data.find("q=");
            if let Some(ref_index) = ref_index {
                let referral_id =
                    &decoded_data[ref_index + 2..query_index.unwrap_or(decoded_data.len())];
                params.push(format!("ref={}", referral_id));
            }
            if let Some(query_index) = query_index {
                let query_id = &decoded_data[query_index + 2..];
                params.push(format!("q={}", query_id));
            }
        }
    }

    let premium_user_status = message.from().map_or(false, |user| user.is_premium);
    if premium_user_status {
        params.push(format!("pr={}", premium_user_status));
    }

    let url = if params.is_empty() {
        URL.to_string()
    } else {
        format!("{}?{}", URL, params.join("&"))
    };

    // Convert the URL string to a reqwest::Url
    let url = reqwest::Url::parse(&url).expect("Invalid URL");

    let inline_kb = InlineKeyboardMarkup::new(
        vec![vec![InlineKeyboardButton::url(
            "Visit Web Page",
            url.clone(),
        )]]
    );

    bot
        .send_message(
            message.chat.id,
            format!("Hello! This is a test bot. You can visit the web page by clicking the button below.\n\n{}\n<a href='{}'>URL</a>", url, url)
        )
        .parse_mode(ParseMode::Html)
        .reply_markup(inline_kb).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // Read the Telegram bot token from the environment variable
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    // Initialize the bot with the token
    let bot = Bot::new(token);

    teloxide::repl(bot.clone(), move |message| {
        let bot = bot.clone();
        async move {
            start(message, bot).await.log_on_error().await;
            respond(())
        }
    }).await;
}
