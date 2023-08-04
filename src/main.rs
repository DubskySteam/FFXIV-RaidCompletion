#![windows_subsystem = "windows"]

mod fetch;
mod playerdata;
mod ui;
mod content;
use std::io::BufRead;
use std::io;
use std::fs::File;
use playerdata::PlayerData;
use regex::Regex;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::um::wincon::GetConsoleWindow;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{ShowWindow, SW_HIDE};

#[tokio::main]
async fn main() {

    #[cfg(target_os = "windows")] {
        windows_console_fix();
    }

    //Channel for msg's between gui && api threads
    let (tx, mut rx) = mpsc::channel::<PlayerData>(100);

    //API Thread
    tokio::spawn(async move {
        let mut p_id: String = String::new();

        match read_id() {
            Ok(id) => {p_id = id;},
            Err(e) => {eprintln!("Couldn't parse config data {}", e)}
        }
        loop {
            match fetch::fetch_data(&p_id).await {
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

#[cfg(target_os = "windows")]
fn windows_console_fix() {
    let window = unsafe {GetConsoleWindow()};
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
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
