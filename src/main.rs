#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

mod player;
mod fetch;
mod ui;
mod content;
use std::io::BufRead;
use std::io;
use std::fs::File;
use std::ptr;
use player::PlayerData;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};
use regex::Regex;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {

    let window = unsafe {GetConsoleWindow()};
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }

    //Channel for msg's between gui && api threads
    let (tx, mut rx) = mpsc::channel::<PlayerData>(100);

    //API Thread
    tokio::spawn(async move {
        let mut P_ID: String = String::new();

        match read_id() {
            Ok(id) => {P_ID = id;},
            Err(e) => {eprintln!("Couldn't parse config data {}", e)}
        }
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
            //println!("Re-fetching");
        }
    });

    //GUI Update task
    let update_task = tokio::spawn(async move {
        while let Some(response) = rx.recv().await {
            //println!("[GUI] TRYING TO UPDATE");
            ui::update(&response);
        }
    });

    //Spawn GUI
    ui::create_ui();

    //GUI terminate await
    let _ = update_task.await;
}

fn read_id() -> io::Result<String> {
    let file = File::open("data/config.data")?;

    // Read the first line
    let first_line = io::BufReader::new(file).lines().next().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "File is empty",
            ))??;

    // Check the line with a regular expression
    let re = Regex::new(r"^ID=(\w+)$").map_err(|_| io::Error::new(
            io::ErrorKind::InvalidData,
            "Regex is invalid",
            ))?;
    let caps = re.captures(&first_line).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid file format",
            ))?;

    // Extract the ID
    let id = caps.get(1).map(|m| m.as_str().to_string()).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid file format",
            ))?;

    Ok(id)
}
