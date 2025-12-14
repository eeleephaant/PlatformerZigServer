#[derive(Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub ready: bool,
}

impl Player {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            ready: false,
        }
    }
    pub fn change_ready(&mut self){
        if self.ready {
            self.ready = false;
        }
        else {
            self.ready = true;
        }
    }
}
