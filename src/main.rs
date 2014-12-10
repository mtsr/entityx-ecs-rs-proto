extern crate ecs;

use std::rc::Rc;
use std::cell::RefCell;

use ecs::{
    // Entity,
    EntityManager,
    System,
    SystemManager,
};

fn main() {
    let mut system_manager = SystemManager::new();
    system_manager.register(TestSystem::new());

    let rc_entity_manager = EntityManager::new();
    { // Scope for rc_entity_manager borrow
        let mut entity_manager = rc_entity_manager.borrow_mut();

        entity_manager.register_component::<Renderable>();
        entity_manager.register_component::<Loud>();

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

struct TestSystem;

impl TestSystem {
    pub fn new() -> TestSystem {
        TestSystem
    }
}

#[deriving(Show)]
struct UpdateArgs;

impl System<UpdateArgs> for TestSystem {
    fn update(&self, entity_manager: Rc<RefCell<EntityManager>>, args: &UpdateArgs) {
        println!("1 {}", args);
        let entity_manager = entity_manager.borrow();

        // include components
        for entity in entity_manager.entities() {
            if let (
                &Some(renderable),
                &Some(loud)
            ) = (
                entity_manager.get_component::<Renderable>(&entity),
                entity_manager.get_component::<Loud>(&entity)
            ) {
                println!("1 {}, {}, {}", entity.id(), renderable, loud);
            }
        }

        // exclude components
        for entity in entity_manager.entities() {
            if let (
                &Some(renderable),
                &None
            ) = (
                entity_manager.get_component::<Renderable>(&entity),
                entity_manager.get_component::<Loud>(&entity)
            ) {
                println!("2 {}, {}", entity.id(), renderable);
            }
        }

        // nested get
        for entity in entity_manager.entities() {
            if let &Some(renderable) = entity_manager.get_component::<Renderable>(&entity) {
                if let &Some(loud) = entity_manager.get_component::<Loud>(&entity) {
                    println!("3 {}, {}, {}", entity.id(), renderable, loud);
                } else {
                    println!("3 {}, {}", entity.id(), renderable);
                }
            }
        }

        // optional component
        for entity in entity_manager.entities() {
            if let (
                &Some(renderable),
                &option_loud
            ) = (
                entity_manager.get_component::<Renderable>(&entity),
                entity_manager.get_component::<Loud>(&entity)
            ) {
                println!("4 {}, {}, {}", entity.id(), renderable, option_loud);
            }
        }
    }
}