use crate::models::model::Instance;

pub struct Scene {
    pub instances: Vec<Instance>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            instances: vec![],
        }
    }

    pub fn add_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
    }
}