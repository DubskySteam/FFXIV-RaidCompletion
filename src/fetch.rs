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

pub async fn fetch_data(id: &str, P_DATA: &mut PlayerData) -> Result<(), Box<dyn std::error::Error>> {
    println!("STARTING TO FETCH");
    let p_data = reqwest::get("https://xivapi.com/character/".to_owned() + id + "?data=AC")
        .await?
        .json::<Player>()
        .await?;

    //println!("pdata:\n{:?}", p_data);
    P_DATA.name = p_data.Character.Name;
    P_DATA.server = p_data.Character.Server;
    P_DATA.datacenter = p_data.Character.DC;
    P_DATA.level = p_data.Character.ActiveClassJob.Level;
    P_DATA.class = p_data.Character.ActiveClassJob.UnlockedState.Name;
    for i in p_data.Achievements.List.iter() {
        P_DATA.achievements.push(i.ID)
    }

    Ok(())
}
