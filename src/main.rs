#![feature(phase)]
#[phase(plugin,link)] extern crate ecs;

use std::rand;
use std::iter::{ IteratorExt };
use std::collections::{
    VecMap,
    Bitv
};

use ecs::{
    World,
    Entity,
    Control,
    EntityManager,
    ComponentManager,
    System,
    TupAppend, // required for components macro
};

fn main() {
    let mut world: World<WorldId1> = World::new();

    world.register_system(Sys);

    world.register_component::<Cmp1>(box VecMap::new());
    world.register_component::<Cmp2>(box VecMap::new());
    world.register_component::<Cmp3>(box VecMap::new());
    world.register_component::<Cmp4>(box VecMap::new());
    world.register_component::<Cmp5>(box VecMap::new());

    for _ in range(0u, 2000000u) {
        let entity = world.create_entity();
        if rand::random::<f32>() > 0.5f32 {
            world.assign_component(&entity, Cmp1);
        }
        if rand::random::<f32>() > 0.3f32 {
            world.assign_component(&entity, Cmp2);
        }
        if rand::random::<f32>() > 0.1f32 {
            world.assign_component(&entity, Cmp3);
        }
        if rand::random::<f32>() > 0.1f32 {
            world.assign_component(&entity, Cmp4);
        }
        if rand::random::<f32>() > 0.1f32 {
            world.assign_component(&entity, Cmp5);
        }
    }

    for _ in range(0u, 10u) {
        world.update_system::<uint, Sys>(&0u);
    }
}

struct WorldId1;

#[deriving(Show)]
struct Cmp1;

#[deriving(Show)]
struct Cmp2;

#[deriving(Show)]
struct Cmp3;

#[deriving(Show)]
struct Cmp4;

#[deriving(Show)]
struct Cmp5;

struct Sys;

impl<WorldId> System<WorldId, Sys> for Sys {
        fn update<A>(&mut self, entity_manager: &EntityManager<WorldId>, component_manager: &ComponentManager<WorldId>, _: &mut Control<WorldId, Sys>, _: &A) {

        let mut counter = 0u;

        // for (_, _, _, _, _) in entities_with_components!(entity_manager, component_manager: without Cmp1 with Cmp2 with Cmp3 with Cmp4 with Cmp5) {
        //     counter += 1;
        // }

        let component_data = (component_manager.get_component_data::<Cmp1>(),)
        .tup_append(component_manager.get_component_data::<Cmp2>())
        .tup_append(component_manager.get_component_data::<Cmp3>())
        .tup_append(component_manager.get_component_data::<Cmp4>())
        .tup_append(component_manager.get_component_data::<Cmp5>());
        let mut with_mask = Bitv::from_elem(component_manager.get_components_length(), false);
        let mut without_mask = Bitv::from_elem(component_manager.get_components_length(), false);
        for tuple in entity_manager.entities().filter(|entity| {
            with_mask.set(component_data.1.index, true);
            with_mask.set(component_data.2.index, true);
            with_mask.set(component_data.3.index, true);
            with_mask.set(component_data.4.index, true);

            without_mask.set(component_data.0.index, true);

            let component_mask = component_manager.get_entity_component_mask(entity);

            if with_mask.intersect(component_mask) || without_mask.difference(component_mask) {
                false
            } else {
                true
            }
        })
        .filter_map(|entity: Entity<WorldId>| {
            if let Some(component) = component_data.1.list.get(&entity.index()) {
                return Some((entity, component));
            } else {
                None
            }
        })
        .filter_map(|tuple| {
            if let Some(component) = component_data.2.list.get(&tuple.0.index()) {
                return Some(tuple.tup_append(component));
            } else {
                None
            }
        })
        .filter_map(|tuple| {
            if let Some(component) = component_data.3.list.get(&tuple.0.index()) {
                return Some(tuple.tup_append(component));
            } else {
                None
            }
        })
        .filter_map(|tuple| {
            if let Some(component) = component_data.4.list.get(&tuple.0.index()) {
                return Some(tuple.tup_append(component));
            } else {
                None
            }
        }) {
            // println!("{}", tuple);
            counter += 1;
        }
    }
}
#[cfg(test)]
mod test {

}