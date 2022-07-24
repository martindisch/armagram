# armagram

Monitor the number of players on your Arma 3 server via Telegram.

## Usage

In order to send you a Telegram message, the application expects two
environment variables to be set:

- `TOKEN` containing the bot token
- `CHAT_ID` with the ID of the chat you have with your bot

You can either make sure these are set in the environment, or copy and modify
`.env.TEMPLATE` to a `.env` file (in the directory you intend to start the
server from) and set these up in there.

Once this is done, just start your Arma server as usual, but make sure to
pipe its output into armagram. If you would like to continue seeing the output
in your terminal, you can use `tee`:

`./arma3server | tee $(tty) | armagram`

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
