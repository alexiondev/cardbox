use std::collections::HashMap;

use crate::gui::ecs::{Component, EntityBuilder};

#[derive(Debug, Default)]
pub struct Screen {
    pub entity_count: usize,
    pub components: HashMap<&'static str, Vec<Component>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            ..Default::default()
        }
    }

    pub fn add_entity(&mut self) -> usize {
        let id = self.entity_count;
        self.entity_count += 1;

        for (_, ref mut component_vec) in self.components.iter_mut() {
            component_vec.push(Component::None);
        }

        id
    }

    pub fn add_component(&mut self, entity: usize, component: Component) {
        let components = self
            .components
            .entry(component.identifier())
            .or_insert(vec![Component::None; self.entity_count]);

        components[entity] = component;
    }

    pub fn new_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }
}
