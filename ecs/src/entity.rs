pub type EntityId = u64;

#[derive(Debug)]
pub struct Entity {
    pub id: EntityId,
    pub tag: String,
    alive: bool,
}

impl Entity {
    pub(crate) const fn new(id: EntityId, tag: String) -> Self {
        Self {
            id,
            alive: true,
            tag,
        }
    }

    pub fn destroy(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
}
