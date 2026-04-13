pub struct EntityManager {
    next_id: u32,
    free_ids: Vec<u32>,
    pub entity_count: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            next_id: 0,
            free_ids: Vec::new(),
            entity_count: 0
        }
    }

    pub fn create_entity(&mut self) -> u32 {
        self.entity_count += 1;

        if let Some(free_id) = self.free_ids.pop() {
            free_id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }

    pub fn delete_entity(&mut self, id: u32) {
        self.entity_count -= 1;

        if !self.free_ids.contains(&id) {
            self.free_ids.push(id);
        }

        // TODO: delete components on {id} index
    }
}