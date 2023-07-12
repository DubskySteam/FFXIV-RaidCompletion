#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

mod player;
mod fetch;
use player::PlayerData;

static P_ID: &str = "48486396";

#[tokio::main]
async fn main() {
    let mut P_DATA: PlayerData = PlayerData {
        name: String::new(),
        level: 0,
        class: String::new(),
        datacenter: String::new(),
        server: String::new(),
        achievements: Vec::new()
    };
    let _ = fetch::fetch_data(P_ID, &mut P_DATA).await;
    println!("Char: {}\nLevel: {}\nClass: {}\nDC: {}\nServer: {}",
             P_DATA.name,
             P_DATA.level,
             P_DATA.class,
             P_DATA.datacenter,
             P_DATA.server,
            );
}
