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
}
