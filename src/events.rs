use ggez::event::{KeyCode, KeyMods};

#[derive(Debug)]
pub enum Event {
    KeyDown(KeyCode, KeyMods, bool),
    KeyUp(KeyCode, KeyMods),
    CloseGame,
}
