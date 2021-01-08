use ggez::input::keyboard::{KeyCode, KeyMods};
use rand::{thread_rng, Rng};
use specs::{join::Join, ReadStorage, System, Write, WriteExpect, WriteStorage};

use crate::entities::EntityType;
use crate::resources::GameState;
use crate::{
    components::*,
    events::Event,
    resources::{EntityQueue, EventQueue},
};

pub struct EventSystem;

// System implementation
impl<'a> System<'a> for EventSystem {
    // Data
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, EntityQueue>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Bar>,
        WriteExpect<'a, GameState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut event_queue,
            mut entity_queue,
            _positions,
            mut velocities,
            balls,
            bars,
            mut game_state,
        ) = data;

        for event in event_queue.events.drain(..) {
            // println!("New event: {:?}", event);
            match event {
                Event::KeyDown(key_code, _key_mods, _is_repeated) => {
                    match (key_code, _is_repeated, _key_mods) {
                        (KeyCode::Tab, false, KeyMods::SHIFT) => {
                            game_state.show_debug ^= true;
                        }
                        (KeyCode::Up, ..) => {
                            for (vel, _) in (&mut velocities, &balls).join() {
                                vel.x += 120.0 * num::signum(vel.x);
                                vel.y += 120.0 * num::signum(vel.y);
                            }
                        }
                        (KeyCode::Down, ..) => {
                            for (vel, _) in (&mut velocities, &balls).join() {
                                vel.x -= 120.0 * num::signum(vel.x);
                                vel.y -= 120.0 * num::signum(vel.y);
                            }
                        }
                        (KeyCode::Right, false, ..) => {
                            for (vel, _) in (&mut velocities, &bars).join() {
                                vel.x = 600.0;
                            }
                        }
                        (KeyCode::Left, false, ..) => {
                            for (vel, _) in (&mut velocities, &bars).join() {
                                vel.x = -600.0;
                            }
                        }
                        (KeyCode::Space, ..) => {
                            entity_queue.push(EntityType::Ball {
                                x: thread_rng().gen_range(-120.0..120.0),
                                y: thread_rng().gen_range(-120.0..120.0),
                            });
                        }
                        (KeyCode::F, false, KeyMods::CTRL) => {
                            game_state.show_fps ^= true;
                        }
                        (KeyCode::L, false, KeyMods::CTRL) => {
                            game_state.sw_frame_limiter ^= true;
                        }
                        (KeyCode::Escape, false, _) => {
                            game_state.continuing = false;
                        }
                        (KeyCode::B, _, _) => {
                            entity_queue.push(EntityType::Brick {
                                x: thread_rng().gen_range(0.0..800.0),
                                y: thread_rng().gen_range(0.0..600.0),
                                health: thread_rng().gen_range(0..50),
                            });
                        }
                        _ => {}
                    }
                }
                Event::KeyUp(key_code, _key_mods) => match key_code {
                    KeyCode::Right => {
                        for (vel, _) in (&mut velocities, &bars).join() {
                            vel.x = 0.0;
                        }
                    }
                    KeyCode::Left => {
                        for (vel, _) in (&mut velocities, &bars).join() {
                            vel.x = 0.0;
                        }
                    }
                    _ => {}
                },
                Event::CloseGame => {
                    game_state.continuing = false;
                }
            }
        }
    }
}
