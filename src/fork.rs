pub struct Fork {
    pub id: u32,
    pub taken: bool,
}

impl Fork {
    pub fn new(id: u32) -> Fork {
        Fork {
            id,
            taken: false,
        }
    }
    
    pub fn take(&mut self) {
        println!("Fork {} is taken!", self.id);
        self.taken = true;
    }
    
    pub fn put(&mut self) {
        println!("Fork {} is put!", self.id);
        self.taken = false;
    }
}