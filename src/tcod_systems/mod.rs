use tcod::{Console, BackgroundFlag, KeyCode, Special};

use ecs::world::{ World, WorldBuilder };
use ecs::system::{ EntityProcess, System, Passive, PassiveEntityProcess };
use ecs::entity::Entity;
use ecs::world::{ EntityData };
use ecs::aspect::Aspect;

use std::collections::TrieMap;

pub struct RenderPassive {
    console: Console,
}

impl RenderPassive {
    pub fn new(world_builder: &WorldBuilder) -> RenderPassive {
        let mut console = Console::init_root(80, 50, "libtcod Rust tutorial", false);

        RenderPassive {
            console: console,
        }
    }
}

impl PassiveEntityProcess for RenderPassive {
    fn process<'a, T: Iterator<&'a Entity>>(&mut self, mut entities: T, world: &World) {
    //     // TODO handle closing
    //     if Console::window_closed() {
    //         return
    //     }

        self.console.clear();
        self.console.put_char(40, 25, '@', BackgroundFlag::Set);

        println!("UPDATE!");
        for entity in entities {
            println!("Render {}", entity);
        }

        Console::flush();
    }
}

impl System for RenderPassive {
    fn activated(&mut self, entity: &Entity, world: &World)
    {
        println!("Activated {}", entity);
    }

    /// Optional method called when an entity is deactivated.
    fn deactivated(&mut self, entity: &Entity, world: &World)
    {
        println!("Deactivated {}", entity);
    }
}

pub struct InputPassive;

impl InputPassive {
    pub fn new(world_builder: &WorldBuilder) -> InputPassive {

        InputPassive
    }
}

impl Passive for InputPassive {
    fn process(&mut self, world: &World) {
        println!("{}", "INPUT!");
        let keypress = Console::wait_for_keypress(true);
//         match keypress.key {
//             Special(key_code::Escape) => exit = true,
//             _ => {}
//         }
    }
}

impl System for InputPassive {}

