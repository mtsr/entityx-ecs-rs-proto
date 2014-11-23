extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;

extern crate ecs;

use std::rc::Rc;

use sdl2_window::Sdl2Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL_3_2;

use std::cell::RefCell;
use piston::{
    RenderArgs,
    UpdateArgs
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

use event::{
    Events,
    Window,
    RenderEvent,
    UpdateEvent,
};

use ecs::{ Entity, EntityManager, System, SystemManager };

struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
    system_manager: SystemManager,
    entity_manager: Rc<RefCell<EntityManager>>,
}

impl App {
    fn new() -> App {
        let mut system_manager = SystemManager::new();
        system_manager.register(box TestSystem);

        let mut entity_manager = EntityManager::new();
        let test_entity1 = Entity::new(entity_manager.downgrade());
        let test_entity2 = Entity::new(entity_manager.downgrade());

        App {
            gl: Gl::new(OpenGL_3_2),
            rotation: 0.0,
            system_manager: system_manager,
            entity_manager: entity_manager,
        }
    }

    fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen.
        context.rgba(0.0,1.0,0.0,1.0).draw(&mut self.gl);

        // Draw a box rotating around the middle of the screen.
        context
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .rect(0.0, 0.0, 50.0, 50.0)
            .rgba(1.0, 0.0, 0.0,1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
    }

    fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
        self.system_manager.update::<TestSystem, &UpdateArgs>(self.entity_manager.clone(), args);
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Create an SDL window.
    let window = Sdl2Window::new(
        OpenGL_3_2,
        piston::WindowSettings::default()
    );

    // Create a new game and run it.
    let mut app = App::new();

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        e.render(|r| app.render(window.borrow_mut().deref_mut(), r));
        e.update(|u| app.update(window.borrow_mut().deref_mut(), u));
    }
}

struct TestSystem;

impl TestSystem {
    pub fn new() -> TestSystem {
        TestSystem
    }
}

impl System for TestSystem {
    fn update<A>(&self, entities: Rc<RefCell<EntityManager>>, args: A) {
    }
}