use crate::models::model::Instance;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Scene {
    pub instances: Vec<Rc<RefCell<Instance>>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { instances: vec![] }
    }

    pub fn add_instance(&mut self, instance: Rc<RefCell<Instance>>) {
        self.instances.push(instance);
    }
}
