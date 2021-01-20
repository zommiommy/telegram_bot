# telegram_bot
Experiments about a Rust telegram bot and auto-publishing of static binaries.

Currently the bot onyl has the `/ip` command which returns the public Ip of where the bot is run.

# Usage
```shell
wget https://github.com/zommiommy/telegram_bot/releases/download/latest/telegram_bot
chmod +x telegram_bot

TELOXIDE_TOKEN="your token goes here" ./telegram_bot
```
