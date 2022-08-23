use log::warn;
use std::fmt::Display;

use crate::PlayerEvent;

/// Maintains the names of the players currently on the server.
pub struct PlayerList {
    players: Vec<String>,
    is_busy: bool,
}

impl PlayerList {
    /// Initializes a new instance.
    pub fn new() -> Self {
        Self {
            players: vec![],
            is_busy: false,
        }
    }

    /// Updates the state with the given event.
    pub fn update(&mut self, event: PlayerEvent) {
        match event {
            PlayerEvent::Connected(player) => self.players.push(player),
            PlayerEvent::Disconnected(player) => {
                if let Some(first_position) =
                    self.players.iter().position(|name| name == &player)
                {
                    self.players.remove(first_position);
                } else {
                    warn!("Player {player} disconnected but was not in player list");
                }
            }
        }

        if self.players.len() >= 3 {
            self.is_busy = true;
        } else if self.players.len() == 0 {
            self.is_busy = false;
        }
    }

    /// Returns the player names.
    pub fn players(&self) -> &[String] {
        &self.players
    }

    /// Returns true when at least 3 players have been active recently.
    pub fn is_busy(&self) -> bool {
        self.is_busy
    }
}

impl Display for PlayerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} players connected", self.players.len())?;
        for player in &self.players {
            write!(f, "\n- {player}")?;
        }

        Ok(())
    }
}

impl Default for PlayerList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_remove() {
        let mut player_list = PlayerList::new();

        player_list.update(PlayerEvent::Connected("johndoe".into()));
        assert_eq!(["johndoe"], player_list.players());
        player_list.update(PlayerEvent::Connected("janedoe".into()));
        assert_eq!(["johndoe", "janedoe"], player_list.players());
        player_list.update(PlayerEvent::Disconnected("johndoe".into()));
        assert_eq!(["janedoe"], player_list.players());
        player_list.update(PlayerEvent::Disconnected("janedoe".into()));
        assert!(player_list.players().is_empty());
    }

    #[test]
    fn duplicates() {
        let mut player_list = PlayerList::new();

        player_list.update(PlayerEvent::Connected("johndoe".into()));
        player_list.update(PlayerEvent::Connected("janedoe".into()));
        player_list.update(PlayerEvent::Connected("johndoe".into()));
        assert_eq!(["johndoe", "janedoe", "johndoe"], player_list.players());

        player_list.update(PlayerEvent::Disconnected("johndoe".into()));
        assert_eq!(["janedoe", "johndoe"], player_list.players());
    }

    #[test]
    fn to_busy_and_back() {
        let mut player_list = PlayerList::new();

        player_list.update(PlayerEvent::Connected("one".into()));
        assert!(player_list.is_busy() == false);
        player_list.update(PlayerEvent::Connected("two".into()));
        assert!(player_list.is_busy() == false);
        player_list.update(PlayerEvent::Connected("three".into()));
        assert!(player_list.is_busy() == true);

        player_list.update(PlayerEvent::Disconnected("one".into()));
        assert!(player_list.is_busy() == true);
        player_list.update(PlayerEvent::Disconnected("two".into()));
        assert!(player_list.is_busy() == true);
        player_list.update(PlayerEvent::Disconnected("three".into()));
        assert!(player_list.is_busy() == false);
    }
}
