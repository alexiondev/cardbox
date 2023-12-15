use super::{Component, Screen};

pub struct EntityBuilder<'a> {
    pub id: usize,
    pub screen: &'a mut Screen,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(screen: &'a mut Screen) -> Self {
        Self {
            id: screen.add_entity(),
            screen,
        }
    }

    pub fn with(self, c: Component) -> Self {
        self.screen.add_component(self.id, c);

        self
    }

    pub fn build(self) -> usize {
        self.id
    }
}
