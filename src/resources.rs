use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<crate::systems::event_types::Event>,
}

pub type EntityQueue = Vec<crate::entities::EntityType>;

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
    pub fn new(window: Window) -> Self {
        GameState {
            this_instant: Instant::now(),
            last_instant: Instant::now(),
            show_fps: true,
            sw_frame_limiter: false,
            sw_frame_limit_fps: 60.0,
            screen_size: (800.0, 600.0),
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

#[derive(Default)]
pub struct AssetCache {
    pub cache: HashMap<String, String>,
}

impl AssetCache {
    pub fn load_assets(&mut self) {
        for _path in std::fs::read_dir("C:/users/dean/git/bouncing-balls/resources")
            // .iter()
            // .filter(|p| p.path().ends_with(".png"))
        {
            // println!("Loading asset: {:?}", path?.path());
            // self.cache.insert(
            //     String::from(path?.path()),
            //     String::from(path?.path()),
            // );
        }
    }
}
