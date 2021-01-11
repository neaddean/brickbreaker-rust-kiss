use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::resource::TextureManager;
use kiss3d::window::{CanvasSetup, NumSamples, Window};
use specs::{DispatcherBuilder, World, WorldExt};

use balz::context::GameContext;
use balz::entities;
use balz::resources::{EntityQueue, GameState};
use balz::systems::*;

fn main() {
    let canvas_config = CanvasSetup { vsync: false, samples: NumSamples::Two };
    let window = Window::new_with_setup("asdf", 800, 600, canvas_config);
    // let window = Window::new("asd");
    let game_context = Rc::new(RefCell::new(GameContext::new(window)));
    {
        let mut game_context = game_context.borrow_mut();
        let window = game_context.window_mut();
        let mut rect = window.add_rectangle(250.0, 650.0);
        rect.set_color(1.0, 0.0, 0.25);
        game_context.store_gfx(rect);
    }

    let ref mut world = World::new();
    world.insert(GameState::new(&mut game_context.borrow_mut()));

    let ref mut dispatcher = DispatcherBuilder::new()
        .with(EventSystem, "events", &[])
        .with_thread_local(EntityCreatorSystem::new(Rc::clone(&game_context)))
        .with_thread_local(PhysicsSystem::new(Rc::clone(&game_context)))
        .with_thread_local(EntityRemovalSystem::new(Rc::clone(&game_context)))
        .with_thread_local(InputSystem::new(Rc::clone(&game_context)))
        .with_thread_local(UpdateRenderablesSystem::new(Rc::clone(&game_context)))
        .with_thread_local(RenderingSystem::new(Rc::clone(&game_context)))
        .build();

    dispatcher.setup(world);

    {
        let mut entity_queue = world.write_resource::<EntityQueue>();
        entity_queue.push(entities::EntityType::Ball { x: 60.0, y: 100.0, r: 25.0 });
        entity_queue.push(entities::EntityType::Ball { x: 25.0, y: 75.0, r: 25.0 });
        entity_queue.push(entities::EntityType::Ball { x: -15.0, y: 90.0, r: 25.0 });
        entity_queue.push(entities::EntityType::Ball { x: -130.0, y: 20.0, r: 25.0 });
        entity_queue.push(entities::EntityType::Bar);

        entity_queue.push(entities::EntityType::Brick {
            x: 50.0,
            y: 50.0,
            health: 2,
        });
        entity_queue.push(entities::EntityType::Brick {
            x: 500.0,
            y: 500.0,
            health: 2,
        });
    }
    {
        let files = std::fs::read_dir("C:/users/dean/rust/bouncing-balls/resources").unwrap();
        for entry in files
        {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if !file_name.ends_with(".png") {
                continue;
            }
            println!("Loading asset: {:?}", file_name);
            TextureManager::get_global_manager(|tm|
                tm.add(entry.path().as_path(), file_name));
        }
    }

    balz::gameloop::run(dispatcher, world);
}

