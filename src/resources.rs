use std::time::{Duration, Instant};

use specs;

use crate::context::GameContext;

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<crate::systems::event_types::Event>,
}

pub type EntityQueue = Vec<crate::entities::EntityType>;
pub type EntityRemovalQueue = Vec<specs::Entity>;


pub struct GameState {
    pub show_fps: bool,
    pub sw_frame_limiter: bool,
    pub sw_frame_limit_fps: f32,
    pub screen_size: (f32, f32),
    pub continuing: bool,
    this_duration: Duration,
    this_instant: Instant,
    last_instant: Instant,
}

impl GameState {
    pub fn new(game_context: &mut GameContext) -> Self {
        // let mut game_context = game_context.borrow_mut();
        let window = game_context.window();
        GameState {
            this_instant: Instant::now(),
            last_instant: Instant::now(),
            show_fps: true,
            sw_frame_limiter: false,
            sw_frame_limit_fps: 60.0,
            screen_size: (window.width() as f32, window.height() as f32),
            this_duration: Default::default(),
            continuing: true,
        }
    }
}

impl GameState {
    pub fn tick(&mut self) {
        self.this_instant = Instant::now();
        self.this_duration = self.this_instant.duration_since(self.last_instant);
        self.last_instant = self.this_instant;
    }

    pub fn this_duration(&self) -> Duration {
        self.this_duration
    }
}
