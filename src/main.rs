use dotenv::dotenv;
use std::{
    env,
    io::{self, BufRead},
};

use armagram::{PlayerEvent, PlayerList, Telegram};

fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN variable is not set");
    let chat_id = env::var("CHAT_ID").expect("CHAT_ID variable is not set");
    let telegram = Telegram::new(token, chat_id);

    let mut player_list = PlayerList::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(event) = PlayerEvent::from_log(&line.unwrap()) {
            player_list.update(event);
            telegram.send(&player_list.to_string());
        }
    }
}
