use std::cell::RefCell;
use std::rc::Rc;

use specs::{Entities, Read, ReadExpect, System, Write, WriteStorage};

use crate::{
    components::*,
    entities::EntityType,
    resources::{EntityQueue, GameState},
};
use crate::context::GameContext;
use crate::resources::AssetCache;

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
        Read<'a, AssetCache>,
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
            _asset_cache,
        ) = data;

        let ref mut game_context = self.game_context.borrow_mut();
        for entity_to_create in entity_queue.drain(..) {
            match entity_to_create {
                EntityType::Ball { x, y } => {
                    // let _asset_name = "/ball.png".to_string();
                    // let dimensions = asset_cache.cache.get(&asset_name).unwrap().dimensions();
                    let mut rect = game_context.window_mut().add_rectangle(50.0, 150.0);
                    rect.set_color(0.0, 1.0, 0.0);
                    game_context.store_gfx(rect);

                    // let gfx_id = 0;
                    let dimensions = (10.0, 10.0);
                    entites
                        .build_entity()
                        .with(
                            Ball {
                                radius: dimensions.0,
                            },
                            &mut ball_storage,
                        )
                        .with(
                            Position {
                                x: 0.0,
                                y: 0.0,
                                z: 10,
                            },
                            &mut positions,
                        )
                        .with(Velocity { x, y }, &mut velocities)
                        .with(Renderable { gfx_id: 0 }, &mut renderables)
                        .build();
                }
                _ => {}
            }
        }
    }
}
