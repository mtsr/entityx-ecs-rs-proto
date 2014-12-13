#![feature(phase)]
#[phase(plugin, link)] extern crate ecs;

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Show;
use std::iter::{ IteratorExt };

use ecs::{
    Entity,
    Control,
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
        entity_manager.assign_component(&test_entity1, Loud(1));
        let test_entity2 = entity_manager.create_entity();
        entity_manager.assign_component(&test_entity2, Loud(2));
        entity_manager.assign_component(&test_entity2, Renderable);
        let test_entity3 = entity_manager.create_entity();
        entity_manager.assign_component(&test_entity3, Loud(3));

        entity_manager.destroy_entity(test_entity1);

        let test_entity4 = entity_manager.create_entity();
        entity_manager.assign_component(&test_entity4, Renderable);
        // entity_manager.assign_component(&test_entity4, Loud);
    }

    for count in range::<uint>(1, 10) {
        system_manager.update::<UpdateArgs, TestSystem>(&rc_entity_manager, &UpdateArgs);
    }
}

#[deriving(Show)]
struct Renderable;

#[deriving(Show)]
struct Loud(int);

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
    fn update<A>(&mut self, entity_manager: &Rc<RefCell<EntityManager>>, control: &mut Control, args: &A) where A: Show {
        println!("1 {}", args);
        let entity_manager = entity_manager.borrow();

        control.build(box |entity_manager: &mut EntityManager, entity: Entity| {
            entity_manager.assign_component(&entity, Renderable);
        });

        for (entity, renderable, option_loud, option_player) in entities_with_components!(entity_manager: with Renderable option Loud option Player) {
            println!("{}, {}, {}, {}", entity.id(), renderable, option_loud, option_player);

            control.modify(entity, box |entity_manager: &mut EntityManager, entity: Entity| {
                entity_manager.assign_component(&entity, Player);
                if let &Some(ref mut loud) = entity_manager.get_component_mut::<Loud>(&entity) {
                    loud.0 = 10;
                };
            });
        }

        for (entity, renderable, option_loud, option_player) in entities_with_components!(entity_manager: with Renderable option Loud option Player) {
            if option_player.is_some() {
                control.destroy(entity)
            };
        }
    }
}