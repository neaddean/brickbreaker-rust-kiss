use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::window::Window;
use specs::{Read, ReadStorage, System, WriteExpect};

use crate::components::*;
use crate::resources;

pub struct RenderingSystem {
    window: Rc<RefCell<Window>>,
    accum: f32,
}

impl RenderingSystem {
    pub fn new(
        window: Rc<RefCell<Window>>,
    ) -> Self {
        RenderingSystem {
            window,
            accum: 0.0,
        }
    }
}

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        Read<'a, resources::AssetCache>,
        WriteExpect<'a, resources::GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_positions,
            _renderables,
            _asset_cache,
            mut game_state,
        ) = data;

        let ref mut window = self.window.borrow_mut();
        if game_state.sw_frame_limiter {
            self.accum += game_state.this_duration().as_secs_f32();
        } else {
            self.accum = 1.0 / game_state.sw_frame_limit_fps;
        }
        while self.accum >= 1.0 / game_state.sw_frame_limit_fps {
            self.accum -= 1.0 / game_state.sw_frame_limit_fps;
            game_state.continuing = window.render();
        }
    }
}
