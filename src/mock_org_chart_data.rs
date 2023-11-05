pub struct GenerateID {
    id: i64,
}

impl GenerateID {
    pub fn new() -> GenerateID {
        GenerateID {
            id: 0
        }
    }

    pub fn get_next_id(&mut self) -> i64 {
        self.id += 1;
        self.id
    }
}