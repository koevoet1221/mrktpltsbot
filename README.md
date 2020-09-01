# `mrktpltsbot`

Periodically polls [Marktplaats](https://www.marktplaats.nl/) for the specified search queries and notifies the user about new items via [Telegram](https://telegram.org/).

[![Crates.io](https://img.shields.io/crates/v/mrktpltsbot?logo=rust)](https://crates.io/crates/mrktpltsbot)
[![Crates.io](https://img.shields.io/crates/l/mrktpltsbot)](https://crates.io/crates/mrktpltsbot)
[![GitHub last commit](https://img.shields.io/github/last-commit/eigenein/mrktpltsbot?logo=github)](https://github.com/eigenein/mrktpltsbot/commits/master)

## Usage

```shell script
mrktpltsbot <bot-token> -c <allowed-chat-id> ...
```

### Supported commands

- `/subscribe <query>`
- `/unsubscribe <subscription ID>`
- `/search <query>` – preview the top 1 result
- For a plain text message the bot will suggest you to subscribe to the search query

### Allow list

The bot allows only the specified chat IDs to interact with itself. To find out a chat ID you can run the bot without a `-c` option and send it a message. It will respond with the chat ID that you have to add to the parameters.

### Monitoring

The bot supports the `--sentry-dsn` option to integrate with [Sentry](https://sentry.io).
