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
    ComponentDatastructure,
};

fn main() {
    let mut system_manager = SystemManager::new();
    system_manager.register(TestSystem::new());

    let rc_entity_manager = EntityManager::new();
    { // Scope for rc_entity_manager borrow
        let mut entity_manager = rc_entity_manager.borrow_mut();

        entity_manager.register_component::<Renderable>(ComponentDatastructure::VecMap);
        entity_manager.register_component::<Loud>(ComponentDatastructure::Vec);
        entity_manager.register_component::<Player>(ComponentDatastructure::HashMap);

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

    for _ in range::<uint>(1, 10) {
        system_manager.update::<UpdateArgs, TestSystem>(&rc_entity_manager, &UpdateArgs);
    }
}

#[deriving(Show)]
struct Renderable;

#[deriving(Show)]
struct Loud(int);

#[deriving(Show)]
struct Player(uint);

struct TestSystem {
    num_players: uint,
}

impl TestSystem {
    pub fn new() -> TestSystem {
        TestSystem {
            num_players: 0
        }
    }
}

#[deriving(Show)]
struct UpdateArgs;

impl System<TestSystem> for TestSystem {
    fn update<A>(&mut self, entity_manager: &Rc<RefCell<EntityManager>>, control: &mut Control<TestSystem>, args: &A) where A: Show {
        println!("1 {}", args);
        let entity_manager = entity_manager.borrow();

        control.build(box |entity_manager: &mut EntityManager, system: &mut TestSystem, entity: Entity| {
            entity_manager.assign_component(&entity, Player(system.num_players));
            system.num_players += 1;
            entity_manager.assign_component(&entity, Renderable);
        });

        for (entity, renderable, option_loud, option_player) in entities_with_components!(entity_manager: with Renderable option Loud option Player) {
            println!("{}, {}, {}, {}", entity.id(), renderable, option_loud, option_player);

            control.modify(entity, box |entity_manager: &mut EntityManager, system: &mut TestSystem, entity: Entity| {
                    if let Some(ref mut loud) = entity_manager.get_component_mut::<Loud>(&entity) {
                    loud.0 = 10;
                };
            });
        }

        for (entity, player) in entities_with_components!(entity_manager: with Player) {
            if let &Player(1) = player {
            } else {
                continue;
            }
            println!("Filtered {}, {}", entity.id(), player);
        }

        // for (entity, option_player) in entities_with_components!(entity_manager: with Player) {
        //     control.destroy(entity)
        // }
    }
}