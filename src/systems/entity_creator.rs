use std::cell::RefCell;
use std::rc::Rc;

use nalgebra as na;
use specs::{Entities, ReadExpect, System, Write, WriteStorage};

use crate::{
    components::*,
    entities::EntityType,
    resources::{EntityQueue, GameState},
};
use crate::context::GameContext;


pub struct EntityCreatorSystem {
    game_context: Rc<RefCell<GameContext>>,
}


impl EntityCreatorSystem {
    pub fn new(
        game_context: Rc<RefCell<GameContext>>,
    ) -> Self {
        EntityCreatorSystem {
            game_context,
        }
    }
}

// System implementation
impl<'a> System<'a> for EntityCreatorSystem {
    // Data
    type SystemData = (
        Write<'a, EntityQueue>,
        Entities<'a>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Bar>,
        WriteStorage<'a, Brick>,
        ReadExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entity_queue,
            entites,
            mut velocities,
            mut positions,
            mut renderables,
            mut ball_storage,
            mut bar_storage,
            mut brick_storage,
            game_state,
        ) = data;

        let ref mut game_context = self.game_context.borrow_mut();
        for entity_to_create in entity_queue.drain(..) {
            match entity_to_create {
                EntityType::Ball { x, y, r } => {
                    let mut rect = game_context.window_mut().add_rectangle(r, r);
                    rect.set_texture_with_name("ball.png");
                    let mut origin = na::geometry::Translation2 {
                        vector: na::Vector2::new(-game_state.screen_size.0 / 4.0,
                                                 game_state.screen_size.1 / 4.0)
                    };
                    origin.vector.x += r / 2.0;
                    origin.vector.y -= r / 2.0;
                    rect.set_local_translation(na::geometry::Translation2::from(origin));
                    let gfx_id = game_context.store_gfx(rect);
                    entites
                        .build_entity()
                        .with(Ball {
                            radius: r,
                        }, &mut ball_storage)
                        .with(Position {
                            x: origin.vector.x,
                            y: origin.vector.y,
                        }, &mut positions)
                        .with(Velocity { x, y }, &mut velocities)
                        .with(Renderable { gfx_id }, &mut renderables)
                        .build();
                }
                EntityType::Bar => {
                    let dimensions = (180.0, 40.0);
                    let mut rect = game_context.window_mut().add_rectangle(dimensions.0, dimensions.1);
                    rect.set_texture_with_name("bar.png");
                    let origin = na::geometry::Translation2 {
                        vector: na::Vector2::new(0.0, -game_state.screen_size.1 / 4.0 + dimensions.1 / 2.0)
                    };
                    rect.set_local_translation(na::geometry::Translation2::from(origin));
                    let gfx_id = game_context.store_gfx(rect);
                    entites
                        .build_entity()
                        .with(Bar {
                            width: dimensions.0,
                            height: dimensions.1,
                        }, &mut bar_storage)
                        .with(Position {
                            x: origin.vector.x,
                            y: origin.vector.y,
                        }, &mut positions, )
                        .with(Velocity { x: 0.0, y: 0.0 }, &mut velocities)
                        .with(Renderable { gfx_id }, &mut renderables)
                        .build();
                }
                EntityType::Brick { x, y, health } => {
                    let dimensions = (120.0, 40.0);
                    let mut rect = game_context.window_mut().add_rectangle(dimensions.0, dimensions.1);
                    rect.set_texture_with_name("green1.png");
                    let origin = na::geometry::Translation2 {
                        vector: na::Vector2::new(x, y)
                    };
                    rect.set_local_translation(na::geometry::Translation2::from(origin));
                    let gfx_id = game_context.store_gfx(rect);
                    entites
                        .build_entity()
                        .with(Brick {
                            width: dimensions.0,
                            height: dimensions.1,
                            health,
                        }, &mut brick_storage)
                        .with(Position { x, y }, &mut positions)
                        .with(Renderable { gfx_id }, &mut renderables)
                        .build();
                }
            }
        }
    }
}
