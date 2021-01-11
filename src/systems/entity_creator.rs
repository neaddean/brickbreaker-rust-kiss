use specs::{Entities, Read, ReadExpect, System, Write, WriteStorage};

use crate::{
    components::*,
    entities::EntityType,
    resources::{EntityQueue, GameState},
};
use crate::resources::AssetCache;

pub struct EntityCreatorSystem;

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
            _renderables,
            mut ball_storage,
            mut bar_storage,
            mut brick_storage,
            game_state,
            _asset_cache,
        ) = data;

        for entity_to_create in entity_queue.drain(..) {
            match entity_to_create {
                EntityType::Ball { x, y } => {
                    let _asset_name = "/ball.png".to_string();
                    // let dimensions = asset_cache.cache.get(&asset_name).unwrap().dimensions();
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
                        // .with(Renderable { asset_name }, &mut renderables)
                        .build();
                }
                EntityType::Bar => {
                    let _asset_name = "/bar.png".to_string();
                    // let dimensions = asset_cache.cache.get(&asset_name).unwrap().dimensions();
                    let dimensions = (10.0, 10.0);
                    entites
                        .build_entity()
                        .with(
                            Bar {
                                width: dimensions.0,
                                height: dimensions.1,
                            },
                            &mut bar_storage,
                        )
                        .with(
                            Position {
                                x: game_state.screen_size.0 / 2.0 - dimensions.1 / 2.0,
                                y: game_state.screen_size.1 - dimensions.1 / 2.0,
                                z: 5,
                            },
                            &mut positions,
                        )
                        .with(Velocity { x: 0.0, y: 0.0 }, &mut velocities)
                        // .with(Renderable { asset_name }, &mut renderables)
                        .build();
                }
                EntityType::Brick { x, y, health } => {
                    let _asset_name = "/green1.png".to_string();
                    // let dimensions = asset_cache.cache.get(&asset_name).unwrap().dimensions();
                    let dimensions = (10.0, 10.0);
                    entites
                        .build_entity()
                        .with(
                            Brick {
                                width: dimensions.0,
                                height: dimensions.1,
                                health,
                            },
                            &mut brick_storage,
                        )
                        .with(Position { x, y, z: 9 }, &mut positions)
                        // .with(Renderable { asset_name }, &mut renderables)
                        .build();
                }
            }
        }
    }
}
