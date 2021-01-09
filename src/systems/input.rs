use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::event::{Action, Key, WindowEvent};
use kiss3d::window::Window;
use specs::{System, Write};

use crate::resources::EventQueue;
use crate::systems::event_types::Event::{KeyDown, KeyUp};

pub struct InputSystem<'a> {
    window: Rc<RefCell<&'a mut Window>>,
    last_pressed: Option<Key>,
}

impl<'a> InputSystem<'a> {
    pub fn new(
        window: Rc<RefCell<&'a mut Window>>,
    ) -> Self {
        InputSystem {
            window,
            last_pressed: None,
        }
    }
}

impl<'a> System<'a> for InputSystem<'_> {
    type SystemData = Write<'a, EventQueue>;

    fn run(&mut self, data: Self::SystemData) {
        let mut event_queue = data;
        let ref mut window = self.window.borrow_mut();

        for event in window.events().iter() {
            println!("{:?}", event.value);
            match event.value {
                WindowEvent::Key(keycode, Action::Press, keymods) => {
                    let repeated = if self.last_pressed.is_some() {
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
