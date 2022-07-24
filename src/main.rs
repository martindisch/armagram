use std::io::{self, BufRead};

mod events;

use events::PlayerEvent;

fn main() {
    let mut players = vec![];

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(event) = PlayerEvent::from_log(&line.unwrap()) {
            match event {
                PlayerEvent::Connected(player) => players.push(player),
                PlayerEvent::Disconnected(player) => {
                    let first_position = players
                        .iter()
                        .position(|name| name == &player)
                        .unwrap();
                    players.remove(first_position);
                }
            }

            println!("{} players connected: {players:#?}", players.len());
        }
    }
}
