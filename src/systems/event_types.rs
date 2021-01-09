use kiss3d::event::{MouseButton, Key, Modifiers};

#[derive(Debug)]
pub enum Event {
    KeyDown(Key, Modifiers, bool),
    KeyUp(Key, Modifiers),
    CloseGame,
    WindowSize(f32, f32),
}
