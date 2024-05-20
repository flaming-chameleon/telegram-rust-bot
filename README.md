Certainly! Below is a professional README for your GitHub repository, including an embedded YouTube video with an image on top. 

### README.md

# Telegram Rust Bot

[![Watch the video](https://img.youtube.com/vi/geXoNSL4OqM/0.jpg)](https://youtu.be/geXoNSL4OqM)

YOutube video: https://youtu.be/geXoNSL4OqM

Welcome to the **Telegram Rust Bot** project! This is a simple yet powerful Telegram bot written in Rust using the [`teloxide`](https://github.com/teloxide/teloxide) library. The bot is designed to interact with users and perform various tasks based on the messages it receives.

## Features

- **Base64 Decoding**: Decode base64 encoded strings from user messages.
- **Referral and Query Handling**: Extract referral and query parameters from decoded data.
- **Dynamic URL Generation**: Generate and send dynamic URLs based on user status and message content.
- **Inline Keyboard**: Send messages with inline keyboard buttons.

## Prerequisites

- Rust (latest stable version recommended)
- Telegram Bot Token

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/flaming-chameleon/telegram-rust-bot.git
    cd telegram-rust-bot
    ```

2. Set up your environment variables:
    ```sh
    export TELEGRAM_BOT_TOKEN=your_telegram_bot_token
    ```

3. Build and run the bot:
    ```sh
    cargo build --release
    cargo run --release
    ```

## Usage

Once the bot is running, you can interact with it on Telegram. Send a message to the bot, and it will respond with a dynamically generated URL based on the message content and user status.

## Code Overview

### Main Function

The main function initializes the bot and starts the message handling loop.

```rust
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
```

### Start Function

The `start` function processes incoming messages and sends responses with inline keyboard buttons.

```rust
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
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License.
