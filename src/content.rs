pub struct Achievements {
    pub name: Vec<String>,
    pub id: Vec<i32>,
    pub status: Vec<bool>
}

impl Achievements {
    pub fn getDungeons() -> Self {
        Achievements {
            name: vec![
                "Howling Eye".to_owned(),
                "Navel".to_owned(),
                "Bowl of Embers".to_owned(),
                "Thornmarch".to_owned(),
                "Whorleater".to_owned(),
                "Striking Tree".to_owned(),
                "Akh Afah Amphithreatre".to_owned(),
            ],
            id: vec![856, 857, 855, 894, 893, 994, 1045],
            status: vec![false, false, false, false, false, false, false]
        } 
    }
    pub fn getTrials() -> Self {
        Achievements {
            name: vec![
                "Howling Eye".to_owned(),
                "Navel".to_owned(),
                "Bowl of Embers".to_owned(),
                "Thornmarch".to_owned(),
                "Whorleater".to_owned(),
                "Striking Tree".to_owned(),
                "Akh Afah Amphithreatre".to_owned(),
            ],
            id: vec![856, 857, 855, 894, 893, 994, 1045],
            status: vec![false, false, false, false, false, false, false]
        } 
    }
    pub fn getRaids() -> Self {
        Achievements {
            name: vec![
                "Binding Coils".to_owned(),
                "Navel".to_owned(),
                "Bowl of Embers".to_owned(),
                "Thornmarch".to_owned(),
                "Whorleater".to_owned(),
                "Striking Tree".to_owned(),
                "Akh Afah Amphithreatre".to_owned(),
            ],
            id: vec![856, 857, 855, 894, 893, 994, 1045],
            status: vec![false, false, false, false, false, false, false]
        } 
    }
}
