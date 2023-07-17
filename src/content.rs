pub struct achievements {
    pub name: Vec<String>,
    pub id: Vec<i32>,
    pub status: Vec<bool>
}

impl achievements {
    pub fn populateTrials() -> Self {
        println!("populateTrials");
        achievements {
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
}
