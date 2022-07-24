use std::io::{self, BufRead};

use armagram::{PlayerEvent, PlayerList};

fn main() {
    let mut player_list = PlayerList::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(event) = PlayerEvent::from_log(&line.unwrap()) {
            player_list.update(event);
            println!("{player_list}");
        }
    }
}
