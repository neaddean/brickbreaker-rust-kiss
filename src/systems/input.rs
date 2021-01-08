use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::event::{Action, WindowEvent};
use specs::{System, Write};

use ggez::Context;
use ggez::event::winit_event::*;
use ggez::input::{keyboard, mouse};
use winit::{dpi, EventsLoop};

use crate::events::Event::{CloseGame, KeyDown, KeyUp};
use crate::ImGuiWrapper;
use crate::resources::EventQueue;

pub struct InputSystem<'a> {
    event_loop: &'a mut EventsLoop,
}

impl<'a> InputSystem<'a> {
    pub fn new(
        event_loop: &'a mut EventsLoop,
    ) -> Self {
        InputSystem {
            event_loop,
        }
    }
}

impl<'a> System<'a> for InputSystem<'_> {
    type SystemData = Write<'a, EventQueue>;

    fn run(&mut self, data: Self::SystemData) {
        let mut event_queue = data;
        // event_queue.events.push(KeyDown(keycode, keymods, repeat));
        // event_queue.events.push(KeyUp(keycode, keymods));

        for event in window.events().iter() {
            match event.value {
                WindowEvent::FramebufferSize(x, y) => {
                    println!("frame buffer size event {}, {}", x, y);
                }
                WindowEvent::MouseButton(button, Action::Press, modif) => {
                    println!("mouse press event on {:?} with {:?}", button, modif);
                    let window_size =
                        na::Vector2::new(window.size()[0] as f32, window.size()[1] as f32);
                    sel_pos = camera.unproject(&last_pos, &window_size);
                    println!(
                        "conv {:?} to {:?} win siz {:?} ",
                        last_pos, sel_pos, window_size
                    );
                }
                WindowEvent::Key(button, Action::Press, _) => {
                    println!("You pressed the button: {:?}", button);
                    println!("Do not try to press escape: the event is inhibited!");
                    event.inhibited = true // override the default keyboard handler
                }
                WindowEvent::Key(button, Action::Release, _) => {
                    println!("You released the button: {:?}", button);
                    println!("Do not try to press escape: the event is inhibited!");
                    event.inhibited = true // override the default keyboard handler
                }
                WindowEvent::Key(key, action, modif) => {
                    println!("key event {:?} on {:?} with {:?}", key, action, modif);
                }
                WindowEvent::CursorPos(x, y, _modif) => {
                    last_pos = na::Point2::new(x as f32, y as f32);
                }
                WindowEvent::Close => {
                    println!("close event");
                }
                _ => {}
            }
        }
        const CROSS_SIZE: f32 = 10.0;
        let up = na::Vector2::new(CROSS_SIZE, 0.0);
        window.draw_planar_line(&(sel_pos - up), &(sel_pos + up), &draw_colour);

        let right = na::Vector2::new(0.0, CROSS_SIZE);
        window.draw_planar_line(&(sel_pos - right), &(sel_pos + right), &draw_colour);
    }
}
}
