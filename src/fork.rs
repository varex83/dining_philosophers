pub struct Fork {
    pub id: u32,
    pub taken: bool,
}

impl Fork {
    pub fn new(id: u32) -> Fork {
        Fork { id, taken: false }
    }

    pub fn take(&mut self) {
        if self.taken {
            panic!("Fork {} is already taken!", self.id);
        }
        println!("Fork {} is taken!", self.id);
        self.taken = true;
    }

    pub fn put(&mut self) {
        if !self.taken {
            panic!("Fork {} is already put!", self.id);
        }
        println!("Fork {} is put!", self.id);
        self.taken = false;
    }
}
