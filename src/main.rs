#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

mod player;
mod fetch;
mod ui;
use player::PlayerData;
use std::env;


#[tokio::main]
async fn main() {
    let mut P_ID: String = "48486396".to_owned();
    //Until UI is finished
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Too many or no arguments given. Using default char_id")
    } else {
        P_ID = args[1].clone();
    }

    //Initial player data
    let mut P_DATA: PlayerData = PlayerData {
        name: String::new(),
        level: 0,
        class: String::new(),
        datacenter: String::new(),
        server: String::new(),
        achievements: Vec::new()
    };
    
    //fetch data from api and do provisional print
    let _ = fetch::fetch_data(&P_ID, &mut P_DATA).await;
    println!("Char: {}\nLevel: {}\nClass: {}\nDC: {}\nServer: {}\nAchievements: {:?}",
             P_DATA.name,
             P_DATA.level,
             P_DATA.class,
             P_DATA.datacenter,
             P_DATA.server,
             P_DATA.achievements
            );

    //For now we create the ui after we fetched the data
    let gui = ui::create_ui();
}
