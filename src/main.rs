extern crate time;

extern crate ecs;

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
    let mut events = Loop::new();

    let mut count = 0us;
    while let Some(event) = events.next() {
        println!("{:?}", event);
        count += 1;
        if count >= 100us {
            break;
        }

        std::io::timer::sleep(std::time::duration::Duration::milliseconds(10));
    }
}

#[derive(Show)]
enum Event {
    Update(u64),
    Render(u64),
}

const FIXED_TIMESTEP_NS: u64 = 1_000_000_000 / 60;
const MAX_LOOPS: u8 = 10;

struct Loop {
    loops: u8,
    accumulated_ns: u64,
    last_ns: u64,
}

impl Loop {
    fn new() -> Loop {
        Loop {
            loops: 0,
            accumulated_ns: 0,
            last_ns: time::precise_time_ns(),
        }
    }
}

impl Iterator for Loop {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        if self.loops >= MAX_LOOPS || self.accumulated_ns < FIXED_TIMESTEP_NS {
            let current_ns = time::precise_time_ns();
            let delta_ns = current_ns - self.last_ns;

            self.accumulated_ns += delta_ns;
            self.last_ns = current_ns;

            self.loops = 0;

            return Some(Event::Render(delta_ns));
        }

        self.accumulated_ns -= FIXED_TIMESTEP_NS;
        self.loops += 1;

        Some(Event::Update(FIXED_TIMESTEP_NS))
    }
}