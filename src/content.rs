use std::{fs::File, io::{self, BufRead}};

pub struct Achievements {
    pub name: Vec<String>,
    pub id: Vec<i32>,
    pub status: Vec<bool>
}

impl Achievements {
    pub fn new() -> Self {
        Achievements { name: Vec::new(), id: Vec::new(), status: Vec::new() }
    }

    pub fn read_data(filename: &str) -> io::Result<Self> {
        let file = File::open("data/".to_owned() + filename + ".data")?;
        let reader = io::BufReader::new(file);

        let mut names = Vec::new();
        let mut ids = Vec::new();
        let mut statuses = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 2 {
                return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid line format",
                        ));
            }
            let name = parts[0].trim().to_string();
            let id = parts[1].trim().parse::<i32>().map_err(|_| io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid id format",
                    ))?;
            names.push(name);
            ids.push(id);
            statuses.push(false);
        }
        Ok(Achievements { name: names, id: ids, status: statuses })
    }
}
