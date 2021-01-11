
use std::cell::RefCell;
use std::rc::Rc;


use specs::{join::Join, ReadStorage, System};
use nalgebra as na;
use crate::components::*;

use crate::context::GameContext;


pub struct UpdateRenderablesSystem {
    game_context: Rc<RefCell<GameContext>>,
}

impl UpdateRenderablesSystem {
    pub fn new(
        game_context: Rc<RefCell<GameContext>>,
    ) -> Self {
        UpdateRenderablesSystem {
            game_context,
        }
    }
}

impl<'a> System<'a> for UpdateRenderablesSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;
        let ref mut game_context = self.game_context.borrow_mut();

        for (pos, rend) in (&positions, &renderables).join() {
            let mut this_obj = game_context.get_gfx(rend.gfx_id);
            let pos = na::geometry::Translation2 { vector: na::Vector2::new(pos.x, pos.y) };
            this_obj.set_local_translation(pos);
        }
    }
}
