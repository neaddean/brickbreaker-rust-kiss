use std::cell::RefCell;
use std::rc::Rc;

use num::traits::Pow;
use specs::{Entities, join::Join, ReadExpect, ReadStorage, System, Write, WriteStorage};

use crate::components::*;
use crate::constants::SIMULATION_DURATION;
use crate::context::GameContext;
use crate::resources;
use crate::resources::EntityRemovalQueue;

pub struct PhysicsSystem {
    game_context: Rc<RefCell<GameContext>>,
    accum: f32,
}

impl PhysicsSystem {
    pub fn new(
        game_context: Rc<RefCell<GameContext>>,
    ) -> Self {
        PhysicsSystem {
            game_context,
            accum: 0.0,
        }
    }
}

struct BarDescriptor {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
}

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Bar>,
        WriteStorage<'a, Brick>,
        ReadExpect<'a, resources::GameState>,
        Write<'a, EntityRemovalQueue>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions,
            mut velocities,
            balls,
            bars,
            mut bricks,
            game_state,
            mut entity_removal_queue,
            entities) = data;

        self.accum += game_state.this_duration().as_secs_f32();
        while self.accum > SIMULATION_DURATION {
            self.accum -= SIMULATION_DURATION;
            let mut bar_desc: Vec<BarDescriptor> = Vec::new();

            // move bar and check if it's at the edge of screen
            for (position, velocity, bar) in (&mut positions, &mut velocities, &bars).join() {
                position.x += velocity.x * SIMULATION_DURATION;
                position.y += velocity.y * SIMULATION_DURATION;

                if position.x - bar.width / 2.0 < -game_state.screen_size.0 / 4.0 {
                    position.x = -game_state.screen_size.0 / 4.0 + bar.width / 2.0;
                } else if position.x + bar.width / 2.0 > game_state.screen_size.0 / 4.0 {
                    position.x = game_state.screen_size.0 / 4.0 - bar.width / 2.0;
                }
                bar_desc.push(BarDescriptor {
                    x: position.x,
                    y: position.y,
                    height: bar.height,
                    width: bar.width,
                });
            }

            // move balls and if they are colliding with anything, reverse velocity
            for (entity, position, velocity, ball) in
            (&entities, &mut positions, &mut velocities, &balls).join()
            {
                position.x += velocity.x * SIMULATION_DURATION;
                position.y += velocity.y * SIMULATION_DURATION;

                if position.x + ball.radius / 2.0 > game_state.screen_size.0 / 4.0 {
                    position.x = game_state.screen_size.0 / 4.0 - ball.radius / 2.0;
                    velocity.x *= -1.0;
                } else if position.x - ball.radius / 2.0 < -game_state.screen_size.0 / 4.0 {
                    position.x = -game_state.screen_size.0 / 4.0 + ball.radius / 2.0;
                    velocity.x *= -1.0;
                }

                if position.y < -game_state.screen_size.1 / 4.0 {
                    entity_removal_queue.push(entity);
                } else if position.y + ball.radius / 2.0 > game_state.screen_size.1 / 4.0 {
                    position.y = game_state.screen_size.1 / 4.0 - ball.radius / 2.0;
                    velocity.y *= -1.0;
                }

                for bar in &bar_desc {
                    if (position.y - ball.radius / 2.0 < bar.y + bar.height / 2.0)
                        & (position.x < bar.x + bar.width / 2.0)
                        & (position.x > bar.x - bar.width / 2.0)
                    {
                        velocity.y *= -1.0;
                        position.y = bar.y + bar.height / 2.0 + ball.radius / 2.0;
                    }
                }
            }
            for (brick_entity, brick) in (&entities, &mut bricks).join() {
                let brick_pos = *positions.get(brick_entity).unwrap();
                for (ball_entity, ball) in (&entities, &balls).join() {
                    let ball_pos = positions.get_mut(ball_entity).unwrap();
                    let (x_side, x_center) = match ball_pos.x {
                        x if x < brick_pos.x - brick.width / 2.0 => {
                            (Some(BrickCollision), brick_pos.x - brick.width / 2.0)
                        }
                        x if x > brick_pos.x + brick.width / 2.0 => {
                            (Some(BrickCollision), brick_pos.x + brick.width / 2.0)
                        }
                        x => (None, x),
                    };

                    let (y_side, y_center) = match ball_pos.y {
                        y if y < brick_pos.y - brick.height / 2.0 => {
                            (Some(BrickCollision), brick_pos.y - brick.height / 2.0)
                        }
                        y if y > brick_pos.y + brick.height / 2.0 => {
                            (Some(BrickCollision), brick_pos.y + brick.height / 2.0)
                        }
                        y => (None, y),
                    };

                    if <f32>::sqrt(
                        (ball_pos.x - x_center).pow(2.0) + (ball_pos.y - y_center).pow(2.0),
                    ) < ball.radius
                    {
                        brick.health = brick.health.checked_sub(1).unwrap_or(0);
                        if brick.health <= 0 {
                            entity_removal_queue.push(brick_entity);
                        }
                        let ball_vel = velocities.get_mut(ball_entity).unwrap();
                        match x_side {
                            Some(_) => {
                                ball_vel.x *= -1.0;
                                ball_pos.x += ball_vel.x * SIMULATION_DURATION;
                            }
                            None => {}
                        };
                        match y_side {
                            Some(_) => {
                                ball_vel.y *= -1.0;
                                ball_pos.y += ball_vel.y * SIMULATION_DURATION;
                            }
                            None => {}
                        };
                    }
                }
            }
        }
    }
}

struct BrickCollision;

// enum BrickCollisionSideX {
//     LEFT,
//     RIGHT,
// }
//
// enum BrickCollisionSideY {
//     TOP,
//     BOTTOM,
// }
