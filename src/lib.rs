//! Various functionality for parsing server logs, maintaining state and
//! sending Telegram messages.

#![deny(missing_docs)]

mod events;
mod state;

pub use events::PlayerEvent;
pub use state::PlayerList;
