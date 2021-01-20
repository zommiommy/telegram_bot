use teloxide::{utils::command::BotCommand, prelude::*};

const BOT_NAME: &str = "HAL";
const IP_ADDRESS: &str = "http://api.ipify.org";

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get the Ip of the server.")]
    Ip,
}

async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Ip => {
            cx.answer_str(
                match reqwest::blocking::get(IP_ADDRESS) {
                    Ok(response) => {
                        match response.text() {
                            Ok(ip) => ip,
                            Err(_err) => {
                                format!("Cannot get the text from the request at {}", IP_ADDRESS)
                            }
                        }
                    }
                    Err(_err) => {
                        format!("Cannot get a response from {}", IP_ADDRESS)
                    }
                }

            ).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env();

    teloxide::commands_repl(bot, BOT_NAME, answer).await;
}