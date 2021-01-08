use specs::WorldExt;

use crate::resources::GameState;

pub fn run(dispatcher: &mut specs::Dispatcher, world: &mut specs::World) {
    loop {
        world.write_resource::<GameState>().tick();
        dispatcher.dispatch(world);
        world.maintain();
        if !world.write_resource::<GameState>().continuing {
            break;
        }
        std::thread::yield_now();
    }
}
