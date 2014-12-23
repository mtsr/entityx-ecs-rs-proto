#![feature(phase)]
extern crate ecs;
#[phase(plugin)] extern crate ecs_macros;

use std::fmt::Show;
use std::iter::{ IteratorExt };

use ecs::{
    Entity,
    Control,
    EntityManager,
    System,
    SystemManager,
    TupAppend, // required for components macro
    ComponentDatastructure,
};

fn main() {
    use std::rand;
    
    let mut system_manager: SystemManager<World1> = SystemManager::new();
    system_manager.register(Sys);

    let mut entity_manager: EntityManager<World1> = EntityManager::new();

    entity_manager.register_component::<Cmp1>(ComponentDatastructure::VecMap);
    entity_manager.register_component::<Cmp2>(ComponentDatastructure::VecMap);
    entity_manager.register_component::<Cmp3>(ComponentDatastructure::VecMap);

    for i in range(0u, 100000u) {
        let entity = entity_manager.create_entity();
        if rand::random::<f32>() > 0.5f32 {
            entity_manager.assign_component(&entity, Cmp1);
        }
        if rand::random::<f32>() > 0.3f32 {
            entity_manager.assign_component(&entity, Cmp2);
        }
        if rand::random::<f32>() > 0.1f32 {
            entity_manager.assign_component(&entity, Cmp3);
        }
    }

    for _ in range::<uint>(1, 10000) {
        system_manager.update::<uint, Sys>(&mut entity_manager, &0u);
    }
}

struct World1;

#[deriving(Show)]
struct Cmp1;

#[deriving(Show)]
struct Cmp2;

#[deriving(Show)]
struct Cmp3;

struct Sys;

impl<Id> System<Id, Sys> for Sys {
    fn update<A>(&mut self, entity_manager: &EntityManager<Id>, control: &mut Control<Id, Sys>, args: &A) where A: Show {

        let mut counter = 0u;

        for (entity, option_cmp2, option_cmp3) in entities_with_components!(entity_manager: without Cmp1 option Cmp2 with Cmp3) {
            counter += 1;
        }
    }
}

#[cfg(test)]
mod test {

}