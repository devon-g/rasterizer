use crate::models::model::Instance;

pub struct Scene<'a> {
    pub instances: Vec<Instance<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Scene { instances: vec![] }
    }

    pub fn add_instance(&mut self, instance: Instance<'a>) {
        self.instances.push(instance);
    }
}
