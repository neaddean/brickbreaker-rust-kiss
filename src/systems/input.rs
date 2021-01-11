use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::event::{Action, Key, WindowEvent};
use specs::{System, Write};

use crate::context::GameContext;
use crate::resources::EventQueue;
use crate::systems::event_types::Event::{KeyDown, KeyUp};

pub struct InputSystem {
    game_context: Rc<RefCell<GameContext>>,
    last_pressed: Option<Key>,
}

impl InputSystem {
    pub fn new(
        game_context: Rc<RefCell<GameContext>>,
    ) -> Self {
        InputSystem {
            game_context,
            last_pressed: None,
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = Write<'a, EventQueue>;

    fn run(&mut self, data: Self::SystemData) {
        let mut event_queue = data;

        let game_context = self.game_context.borrow_mut();
        let window = game_context.window();

        for event in window.events().iter() {
            println!("{:?}", event.value);
            match event.value {
                WindowEvent::Key(keycode, Action::Press, keymods) => {
                    let repeated = if (self.last_pressed.is_some())
                        & (keycode != Key::LControl)
                        & (keycode != Key::RControl)
                        & (keycode != Key::LShift)
                        & (keycode != Key::RShift)
                    {
                        self.last_pressed == Some(keycode)
                    } else {
                        false
                    };
                    self.last_pressed = Some(keycode);
                    event_queue.events.push(KeyDown(keycode, keymods, repeated));
                }
                WindowEvent::Key(keycode, Action::Release, keymods) => {
                    event_queue.events.push(KeyUp(keycode, keymods));
                }
                _ => {}
            }
        }
    }
}
