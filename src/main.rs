#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

mod player;
mod fetch;
mod ui;
mod content;
use std::env;
use player::PlayerData;
use tokio::task;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    //Channel for msg's between gui && api threads
    let (tx, mut rx) = mpsc::channel::<PlayerData>(100);


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
