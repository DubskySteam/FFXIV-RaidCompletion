use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Player {
    Character: Box<p_character> 
}

#[derive(Deserialize, Debug)]
struct p_character {
    DC: String,
    Name: String,
    Server: String,
    Title: i32
}

pub async fn fetch_data() -> Result<(), Box<dyn std::error::Error>> {
    println!("STARTING TO FETCH");
    let p_data = reqwest::get("https://xivapi.com/character/48486396")
        .await?
        .json::<Player>()
        .await?;

    println!("{:?}", p_data);

    Ok(())
}
