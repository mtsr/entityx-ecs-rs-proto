#![feature(phase)]

#[phase(plugin, link)]
extern crate ecs;
extern crate tcod;

use ecs::world::{ WorldBuilder, Components };
use ecs::entity::Entity;
use ecs::system::{ EntitySystem, PassiveEntitySystem };
use ecs::aspect::Aspect;

use ecs::system::{ EntityProcess, System, Passive };
use ecs::world::{ EntityData };

use tcod_systems::{ RenderPassive, InputPassive };

mod tcod_systems;

component! {
    Position {
        x: f32,
        y: f32 // Due to macro parsing problems, trailing commas do not work.
    }

    Velocity {
        dx: f32,
        dy: f32
    }
    // etc.
}

new_type! {
    Team(int);
    Experience(int);
    // etc.
}

feature! {
    Renderable;
    // etc.
}

pub struct PrintEntityID;

impl EntityProcess for PrintEntityID
{
    fn process<'a, T: Iterator<&'a Entity>>(&self, mut entities: T, entity_data: &mut EntityData)
    {
        for entity in entities
        {
            println!("Processed Entity: {}", entity.get_id());
        }
    }
}

impl System for PrintEntityID {}

fn main() {
    let mut builder = WorldBuilder::new();

    builder.register_component::<Position>();
    builder.register_component::<Velocity>();
    builder.register_component::<Team>();
    builder.register_component::<Renderable>();

    let print_entity_ID = PrintEntityID;
    let print_entity_ID_system = EntitySystem::new(print_entity_ID, Aspect::nil());
    builder.register_system(box print_entity_ID_system);

    let render_passive = RenderPassive::new(&builder);
    let render_passive_system = PassiveEntitySystem::new(render_passive, Aspect::nil());
    builder.register_passive("render", box render_passive_system);

    let input_passive = InputPassive::new(&builder);
    builder.register_passive("input", box input_passive);

    let mut world = builder.build();

    let entity = world.build_entity(
        |c: &mut Components, e: Entity| {
            c.add(&e, Position { x: 5.0, y: 2.0 });
            c.add(&e, Velocity { dx: 0.0, dy: 0.0 });
            c.add(&e, Team(1));
        }
    );

    world.update_passive("render");
    world.update();
    world.update_passive("render");

    world.modify_entity(entity,
        |c: &mut Components, e: Entity| {
            c.add(&e, Renderable);
            c.set(&e, Team(2));
            c.remove::<Velocity>(&e);
        }
    );

    world.update_passive("input");

    world.update();
    world.update_passive("render");

    world.delete_entity(&entity);
}
