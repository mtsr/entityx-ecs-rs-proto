#![feature(phase)]
#[phase(plugin, link)] extern crate ecs;

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Show;
use std::iter::{ IteratorExt };

use ecs::{
    // Entity,
    EntityManager,
    System,
    SystemManager,
    TupAppend, // required for components macro
};

fn main() {
    let mut system_manager = SystemManager::new();
    system_manager.register(TestSystem::new());

    let rc_entity_manager = EntityManager::new();
    { // Scope for rc_entity_manager borrow
        let mut entity_manager = rc_entity_manager.borrow_mut();

        entity_manager.register_component::<Renderable>();
        entity_manager.register_component::<Loud>();
        entity_manager.register_component::<Player>();

        let test_entity1 = entity_manager.create_entity();
        entity_manager.assign_component(&test_entity1, Renderable);
        entity_manager.assign_component(&test_entity1, Loud);
        let test_entity2 = entity_manager.create_entity();
        entity_manager.assign_component(&test_entity2, Renderable);
        let test_entity3 = entity_manager.create_entity();
        entity_manager.assign_component(&test_entity3, Loud);

        entity_manager.destroy_entity(test_entity1);

        let test_entity4 = entity_manager.create_entity();
        entity_manager.assign_component(&test_entity4, Renderable);
        // entity_manager.assign_component(&test_entity4, Loud);
    }

    system_manager.update::<UpdateArgs, TestSystem>(rc_entity_manager.clone(), &UpdateArgs);
}

#[deriving(Show)]
struct Renderable;

#[deriving(Show)]
struct Loud;

#[deriving(Show)]
struct Player;

struct TestSystem;

impl TestSystem {
    pub fn new() -> TestSystem {
        TestSystem
    }
}

#[deriving(Show)]
struct UpdateArgs;

impl System for TestSystem {
    fn update<A>(&self, entity_manager: Rc<RefCell<EntityManager>>, args: &A) where A: Show {
    fn update<A>(&mut self, entity_manager: Rc<RefCell<EntityManager>>, args: &A) where A: Show {
        println!("1 {}", args);
        let entity_manager = entity_manager.borrow();

        for (entity, renderable, option_loud) in entities_with_components!(entity_manager: with Renderable option Loud without Player) {
            println!("{}, {}, {}", entity.id(), renderable, option_loud);
        }
    }
}