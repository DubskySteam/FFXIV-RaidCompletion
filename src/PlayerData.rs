pub struct PlayerData {
    pub name: String,
    pub level: i32,
    pub class: String,
    pub datacenter: String,
    pub server: String,
    pub achievements: Vec<i32>
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData {
            name: String::new(),
            level: 0,
            class: String::new(),
            datacenter: String::new(),
            server: String::new(),
            achievements: Vec::new()
        }
    }
}
