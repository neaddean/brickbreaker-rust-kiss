use std::cell::RefCell;
use std::rc::Rc;

use kiss3d::light::Light;
use kiss3d::window::Window;
use specs::{DispatcherBuilder, World, WorldExt};

use balz::entities;
use balz::resources::{AssetCache, EntityQueue, GameState};
use balz::systems::{EntityCreatorSystem, EventSystem, InputSystem, PhysicsSystem, RenderingSystem};
use std::borrow::Borrow;

fn main() {
    let window = Window::new("asdf");
    let window = Rc::new(RefCell::new(window));
    // let mut camera = kiss3d::planar_camera::FixedView::new();
    {
        let mut window_l = window.borrow_mut();
        // window_l.set_light(Light::StickToCamera);
        let mut rect = window_l.add_rectangle(50.0, 150.0);
        rect.set_color(0.0, 1.0, 0.0);
    }


    let ref mut world = World::new();
    world.insert(GameState::new(window.borrow()));

    let ref mut dispatcher = DispatcherBuilder::new()
        .with(EventSystem, "events", &[])
        .with(EntityCreatorSystem, "entites", &["events"])
        .with(PhysicsSystem::default(), "physics", &["entites"])
        .with_thread_local(InputSystem::new(
            Rc::clone(&window),
        ))
        .with_thread_local(RenderingSystem::new(
            Rc::clone(&window),
        ))
        .build();

    dispatcher.setup(world);

    {
        let mut entity_queue = world.write_resource::<EntityQueue>();
        entity_queue.push(entities::EntityType::Ball { x: 60.0, y: 100.0 });
        entity_queue.push(entities::EntityType::Ball { x: 25.0, y: 75.0 });
        entity_queue.push(entities::EntityType::Ball { x: -15.0, y: 90.0 });
        entity_queue.push(entities::EntityType::Ball { x: -130.0, y: 20.0 });
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
        let mut asset_cache = world.write_resource::<AssetCache>();
        asset_cache.load_assets();
    }

    balz::gameloop::run(dispatcher, world);
}
