#![allow(unstable)]

extern crate time;

extern crate glutin;
extern crate nalgebra;

extern crate ecs;

use std::collections::{ VecMap, HashMap };

use nalgebra::{ Vec2 };

use glutin::{ Window, PollEventsIterator, ElementState, VirtualKeyCode };

use ecs::{
    World,
    Entity,
    Control,
    EntityManager,
    ComponentManager,
    System,
    TupAppend, // required for components macro
};

use handle_manager::{ HandleManager, Handle };

mod handle_manager;

struct WorldId1;

struct Position(Vec2<f64>);
struct Velocity(Vec2<f64>);
struct Camera;

fn main() {
    let mut world: World<WorldId1> = World::new();

    world.register_component::<Position>(Box::new(VecMap::new()));
    world.register_component::<Velocity>(Box::new(VecMap::new()));
    world.register_component::<Handle<WorldId1, Sprite>>(Box::new(VecMap::new()));
    world.register_component::<Camera>(Box::new(HashMap::new()));

    let ball = world.create_entity();
    world.assign_component(&ball, Position(Vec2::new(100f64, 100f64)));
    world.assign_component(&ball, Velocity(Vec2::new(10f64, 10f64)));

    let camera = world.create_entity();
    world.assign_component(&camera, Position(Vec2::new(100f64, 100f64)));
    world.assign_component(&camera, Camera);

    let mut render_system = RenderSystem::<WorldId1>::new();

    let window = glutin::WindowBuilder::new()
        // vsync doesn't work on OSX yet
        .with_vsync()
        .build_strict()
        .unwrap();

    unsafe { window.make_current() };

    let mut main_loop = MainLoop::new(&window);

    while let Some(event) = main_loop.next() {
        match event {
            Event::Update(dt, mut events) => {
                // println!("Update");
                if !update(&mut world, dt, events) {
                    break;
                }
            },
            Event::Render(dt, window) => {
                // println!("Render");
                // TODO interpolation
                render_system.render(&world, dt, window);
            },
        }
    }
}

#[allow(unused_variables)]
fn update<WorldId>(world: &mut World<WorldId>, dt: u64, events: PollEventsIterator) -> bool {
    true
}

struct RenderSystem<WorldId> {
    sprite_manager: HandleManager<WorldId, Sprite>,
}

impl<WorldId> RenderSystem<WorldId> {
    fn new() -> RenderSystem<WorldId> {
        RenderSystem {
            sprite_manager: HandleManager::new(),
        }
    }

    fn create_sprite(&mut self, sprite: Sprite) -> Handle<WorldId, Sprite> {
        self.sprite_manager.create(sprite)
    }

    fn render(&mut self, world: &World<WorldId>, dt: u64, window: &Window) {
        
    }
}

struct Sprite;

enum Event<'a> {
    Update(u64, PollEventsIterator<'a>),
    Render(u64, &'a Window),
}

const FIXED_TIMESTEP_NS: u64 = 1_000_000_000 / 60;
const MAX_LOOPS: u8 = 10;

struct MainLoop<'a> {
    loops: u8,
    accumulated_ns: u64,
    last_ns: u64,

    window: &'a Window,
}

impl<'a> MainLoop<'a> {
    fn new(window: &'a Window) -> MainLoop<'a> {
        MainLoop {
            loops: 0,
            accumulated_ns: 0,
            last_ns: time::precise_time_ns(),
            window: window,
        }
    }
}

impl<'a> Iterator for MainLoop<'a> {
    type Item = Event<'a>;

    // TODO consider pushing fixed timesteps into systems
    // that need it, such as physics
    fn next(&mut self) -> Option<Event<'a>> {
        if self.window.is_closed() {
            return None;
        }

        if self.loops >= MAX_LOOPS || self.accumulated_ns < FIXED_TIMESTEP_NS {
            let current_ns = time::precise_time_ns();
            let delta_ns = current_ns - self.last_ns;

            self.accumulated_ns += delta_ns;
            self.last_ns = current_ns;

            self.loops = 0;

            return Some(Event::Render(delta_ns, self.window));
        }

        // this is after a render
        if self.loops == 0 {
            self.window.swap_buffers();
        }

        // TODO "proper" input handling based on timestamps
        let events = self.window.poll_events();

        self.accumulated_ns -= FIXED_TIMESTEP_NS;
        self.loops += 1;

        Some(Event::Update(FIXED_TIMESTEP_NS, events))
    }
}