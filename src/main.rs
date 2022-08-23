use dotenv::dotenv;
use log::{info, LevelFilter};
use std::{
    env,
    io::{self, BufRead},
};

use armagram::{PlayerEvent, PlayerList, Telegram};

fn main() {
    dotenv().ok();

    systemd_journal_logger::init().expect("Unable to initialize systemd logs");
    log::set_max_level(LevelFilter::Info);
    info!("armagram starting");

    let token = env::var("TOKEN").expect("TOKEN variable is not set");
    let chat_id = env::var("CHAT_ID").expect("CHAT_ID variable is not set");
    let telegram = Telegram::new(token, chat_id);

    let mut player_list = PlayerList::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(event) = PlayerEvent::from_log(&line.unwrap()) {
            player_list.update(event);

            if player_list.is_busy() {
                telegram.send(&player_list.to_string());
            }
        }
    }
}
