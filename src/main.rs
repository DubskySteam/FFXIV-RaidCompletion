#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

mod player;
mod fetch;
mod ui;
use player::PlayerData;
use tokio::task;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::sleep;
use std::env;

#[tokio::main]
async fn main() {
    //Channel for msg's between gui && api threads
    let (tx, mut rx) = mpsc::channel::<PlayerData>(100);

    let mut P_ID: String = "48486396".to_owned();
    //Until UI is finished
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Too many or no arguments given. Using default char_id")
    } else {
        P_ID = args[1].clone();
    }

    //API Thread
    tokio::spawn(async move {
        loop {
            match fetch::fetch_data(&P_ID).await {
                Ok(reponse) => {
                    if tx.send(reponse).await.is_err() {
                        eprintln!("Error communicating with gui");
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Error while executing api calls! {}", e);
                }
            }
            //Fetch every 60 seconds 
            sleep(Duration::from_secs(60)).await;
            println!("Re-fetching");
        }
    });

    //GUI Update task
    let update_task = tokio::spawn(async move {
        while let Some(response) = rx.recv().await {
            println!("[GUI] TRYING TO UPDATE");
            ui::update(&response);
        }
    });

    //Spawn GUI
    ui::create_ui();

    //GUI terminate await
    let _ = update_task.await;
}
