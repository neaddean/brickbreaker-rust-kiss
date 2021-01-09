use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;
use kiss3d::window::Window;
use specs::{join::Join, Read, ReadStorage, System, WriteExpect};

use crate::components::*;

pub struct RenderingSystem<'a> {
    window: Rc<RefCell<&'a mut Window>>,
    accum: f32,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(
        window: Rc<RefCell<&'a mut Window>>,
    ) -> Self {
        RenderingSystem {
            window,
            accum: 0.0,
        }
    }
}

impl RenderingSystem<'_> {
    fn draw_text(&mut self, text_string: &str, x: f32, y: f32, color: graphics::Color) -> f32 {
    }
}

impl<'a> System<'a> for RenderingSystem<'_> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        Read<'a, resources::AssetCache>,
        WriteExpect<'a, resources::GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables, asset_cache, mut game_state) = data;

        if game_state.sw_frame_limiter {
            self.accum += game_state.this_duration().as_secs_f32();
        } else {
            self.accum = SW_FRAME_RATE_DURATION;
        }
        while self.accum >= SW_FRAME_RATE_DURATION {
            self.accum -= SW_FRAME_RATE_DURATION;

            self.ctx.borrow_mut().timer_context.tick();

            graphics::clear(
                *self.ctx.borrow_mut(),
                graphics::Color::new(0.0, 0.0, 0.0, 1.0),
            );

            let mut rendering_batches: HashMap<u8, HashMap<String, Vec<graphics::DrawParam>>> =
                HashMap::new();

            for (position, renderable) in (&positions, &renderables).join() {
                rendering_batches
                    .entry(position.z)
                    .or_default()
                    .entry(renderable.asset_name.to_string())
                    .or_default()
                    .push(
                        graphics::DrawParam::new()
                            .dest(na::Point2::new(position.x, position.y))
                            .offset(na::Point2::new(0.5, 0.5)),
                    );
            }

            // Iterate spritebatches ordered by z and actually render each of them
            for (_z, group) in rendering_batches
                .iter()
                .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            {
                for (asset_name, draw_params) in group {
                    let texture = asset_cache.cache.get(asset_name).unwrap().clone();
                    let mut sprite_batch = SpriteBatch::new(texture);

                    for draw_param in draw_params.iter() {
                        sprite_batch.add(*draw_param);
                    }

                    graphics::draw(
                        *self.ctx.borrow_mut(),
                        &sprite_batch,
                        graphics::DrawParam::new(),
                    )
                        .unwrap();
                }
            }
            let text_line: f32 = 0.0;
            if game_state.show_fps {
                self.draw_text(
                    format!("{:0.2}", ggez::timer::fps(*self.ctx.borrow_mut())).as_str(),
                    text_line + 2.0,
                    2.0,
                    graphics::Color::new(0.0, 1.0, 0.0, 1.0),
                );

                // self.draw_text(format!("SW limit: {}",
                //                        match game_state.sw_frame_limiter {
                //                            true => { "on" }
                //                            false => { "off" }
                //                        }).as_str(),
                //                2.0, text_line + 2.0,
                //                graphics::Color::new(0.0, 1.0, 0.0, 1.0));
            }
            graphics::draw_queued_text(
                *self.ctx.borrow_mut(),
                graphics::DrawParam::new().dest(na::Point2::new(0.0, 0.0)),
                None,
                graphics::FilterMode::Linear,
            )
                .unwrap();

            if game_state.show_debug {
                self.imgui_wrapper
                    .borrow_mut()
                    .render(*self.ctx.borrow_mut(), &mut game_state);
            }

            graphics::present(*self.ctx.borrow_mut()).unwrap();
        }
    }
}
