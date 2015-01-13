#![allow(unstable)]

pub struct HandleManager<Id, T> {
    vec: Vec<T>
}

impl<Id, T> HandleManager<Id, T> {
    pub fn new() -> HandleManager<Id, T>{
        HandleManager {
            vec: Vec::new()
        }
    }

    pub fn get<'r>(&'r self, h: &Handle<Id, T>) -> &'r T {
        let index = h.id;
        &self.vec[index]
    }

    pub fn set(&mut self, h: &Handle<Id, T>, t: T) {
        let index = h.id;
        self.vec[index] = t;
    }

    pub fn create(&mut self, t: T) -> Handle<Id, T> {
        self.vec.push(t);
        Handle {
            id: self.vec.len() - 1
        }
    }
}

pub struct Handle<Id, T> {
    id: usize,
}

#[cfg(test)]
mod tests {
    use handle_manager::HandleManager;

    #[test]
    fn something() {
        struct A; struct B;
        let mut s1 = HandleManager::<A, usize>::new();
        let s2 = HandleManager::<B, usize>::new();

        let handle1 = s1.create(5);
        s1.get(&handle1);

        // s2.get(&handle1); // won't compile, since A != B
    }
}