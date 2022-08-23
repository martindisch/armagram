use lazy_static::lazy_static;
use regex::Regex;

/// The connection state change and affected player name.
#[derive(Debug, PartialEq, Eq)]
pub enum PlayerEvent {
    /// The player has connected.
    Connected(String),
    /// The player has disconnected.
    Disconnected(String),
}

impl PlayerEvent {
    /// Constructs a connection event from the given log line if applicable.
    pub fn from_log(line: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Player (.+) (connected|disconnected)(.+)?\.")
                    .unwrap();
        }

        if let Some(captures) = RE.captures(line) {
            if &captures[2] == "connected" {
                return Some(Self::Connected(captures[1].to_string()));
            } else {
                return Some(Self::Disconnected(captures[1].to_string()));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_name() {
        let event = PlayerEvent::from_log(
            "12:46:02 Player johndoe connected (id=12345678987654321).",
        );

        assert_eq!(Some(PlayerEvent::Connected("johndoe".to_string())), event);
    }

    #[test]
    fn long_name() {
        let event = PlayerEvent::from_log(
            "12:46:02 Player John Doe connected (id=12345678987654321).",
        );

        assert_eq!(
            Some(PlayerEvent::Connected("John Doe".to_string())),
            event
        );
    }

    #[test]
    fn disconnect() {
        let event =
            PlayerEvent::from_log("12:46:02 Player johndoe disconnected.");

        assert_eq!(
            Some(PlayerEvent::Disconnected("johndoe".to_string())),
            event
        );
    }

    #[test]
    fn battleye() {
        let event = PlayerEvent::from_log(
            "12:46:21 BattlEye Server: Player #0 johndoe disconnected",
        );

        assert_eq!(None, event);
    }

    #[test]
    fn other() {
        let event = PlayerEvent::from_log(
            "12:46:31 All users disconnected, waiting for users.",
        );

        assert_eq!(None, event);
    }
}
