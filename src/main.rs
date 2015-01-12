extern crate time;

extern crate glutin;

extern crate ecs;

use glutin::{ Window, PollEventsIterator };

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
    let window = glutin::Window::new().unwrap();

    unsafe { window.make_current() };

    let mut events = Loop::new(&window);

    loop {
        match events.next() {
            Some(Event::Update(dt, events)) => {
            },
            Some(Event::Render(ref window, dt)) => {
            },
            None => {
                break;
            }
        }
    }
}

enum Event<'a> {
    Update(u64, PollEventsIterator<'a>),
    Render(&'a Window, u64),
}

const FIXED_TIMESTEP_NS: u64 = 1_000_000_000 / 60;
const MAX_LOOPS: u8 = 10;

struct Loop<'a> {
    loops: u8,
    accumulated_ns: u64,
    last_ns: u64,

    window: &'a Window,
}

impl<'a> Loop<'a> {
    fn new(window: &'a Window) -> Loop<'a> {
        Loop {
            loops: 0,
            accumulated_ns: 0,
            last_ns: time::precise_time_ns(),
            window: window,
        }
    }
}

impl<'a> Iterator for Loop<'a> {
    type Item = Event<'a>;

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

            return Some(Event::Render(self.window, delta_ns));
        }

        // this is after a render
        if self.loops == 0 {
            self.window.swap_buffers();
        }

        let events = self.window.poll_events();

        self.accumulated_ns -= FIXED_TIMESTEP_NS;
        self.loops += 1;

        Some(Event::Update(FIXED_TIMESTEP_NS, events))
    }
}