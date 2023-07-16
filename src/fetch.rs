use serde::Deserialize;
use crate::player::PlayerData;

#[derive(Deserialize, Debug)]
struct Player {
    Character: Box<p_character>,
    Achievements: Box<p_achievements>
}

#[derive(Deserialize, Debug)]
struct p_achievements {
    List: Vec<ListElement>
}

#[derive(Deserialize, Debug)]
struct ListElement {
    ID: i32
}

#[derive(Deserialize, Debug)]
struct ActiveClassJob {
    UnlockedState: Box<UnlockedState>,
    Level: i32
}

#[derive(Deserialize, Debug)]
struct UnlockedState {
    Name: String
}

#[derive(Deserialize, Debug)]
struct p_character {
    ActiveClassJob: Box<ActiveClassJob>,
    DC: String,
    Name: String,
    Server: String,
    Title: i32
}

#[derive(Debug)]
pub struct ApiError {
    message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    fn new(message: &str) -> Self {
        ApiError {
            message: message.to_owned(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for ApiError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self { message: error.to_string() }
    }
}

pub async fn fetch_data(id: &str) -> Result<PlayerData, ApiError> {
    println!("STARTING TO FETCH");
    let p_data = reqwest::get(format!("https://xivapi.com/character/{}?data=AC", id))
        .await
        .map_err(|err| ApiError::new(&format!("{}", err)))?
        .json::<Player>()
        .await
        .map_err(|err| ApiError::new(&format!("{}", err)))?;
    println!("DONE FETCHING");

    let mut P_DATA: PlayerData = PlayerData::new();
    //println!("pdata:\n{:?}", p_data);
    P_DATA.name = p_data.Character.Name;
    P_DATA.server = p_data.Character.Server;
    P_DATA.datacenter = p_data.Character.DC;
    P_DATA.level = p_data.Character.ActiveClassJob.Level;
    P_DATA.class = p_data.Character.ActiveClassJob.UnlockedState.Name;
    for i in p_data.Achievements.List.iter() {
        P_DATA.achievements.push(i.ID)
    }

    Ok(P_DATA)
}
