use std::fmt::Display;

use crate::PlayerEvent;

/// Maintains the names of the players currently on the server.
pub struct PlayerList {
    players: Vec<String>,
}

impl PlayerList {
    /// Initializes a new instance.
    pub fn new() -> Self {
        Self { players: vec![] }
    }

    /// Updates the state with the given event.
    pub fn update(&mut self, event: PlayerEvent) {
        match event {
            PlayerEvent::Connected(player) => self.players.push(player),
            PlayerEvent::Disconnected(player) => {
                let first_position = self
                    .players
                    .iter()
                    .position(|name| name == &player)
                    .expect("Disconnected player that wasn't connected");
                self.players.remove(first_position);
            }
        }
    }

    /// Returns the player names.
    pub fn players(&self) -> &[String] {
        &self.players
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
}
