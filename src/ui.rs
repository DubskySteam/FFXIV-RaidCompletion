use dioxus::prelude::*;
use dioxus_desktop::*;
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use crate::content::Achievements;
use crate::playerdata::PlayerData;

struct PlayerDataMutex {
    player_data: Mutex<PlayerData>,
    player_achievements: Mutex<[Achievements; 3]>
}

impl PlayerDataMutex {
    fn new() -> Self {
        Self {
            player_data: Mutex::new(PlayerData::new()),
            player_achievements: Mutex::new([
                Achievements::new(),
                Achievements::new(),
                Achievements::new()
            ])
        }
    }
}

lazy_static! {
    static ref PDM: Arc<PlayerDataMutex> = Arc::new(PlayerDataMutex::new());
}

pub fn update(new_data: &PlayerData) {
    if PDM.player_achievements.lock().unwrap()[0].status.len() <= 1 {
        match Achievements::read_data("dungeons") {
            Ok(result) => PDM.player_achievements.lock().unwrap()[0] = result,
            Err(e) => eprintln!("Error >> {}", e),
        }
        match Achievements::read_data("trials") {
            Ok(result) => PDM.player_achievements.lock().unwrap()[1] = result,
            Err(e) => eprintln!("Error >> {}", e),
        }
        match Achievements::read_data("raids") {
            Ok(result) => PDM.player_achievements.lock().unwrap()[2] = result,
            Err(e) => eprintln!("Error >> {}", e),
        }
    }
    PDM.player_data.lock().unwrap().name = new_data.name.clone();
    PDM.player_data.lock().unwrap().level = new_data.level;
    PDM.player_data.lock().unwrap().class = new_data.class.clone();
    PDM.player_data.lock().unwrap().server = new_data.server.clone();
    PDM.player_data.lock().unwrap().datacenter = new_data.datacenter.clone();
    PDM.player_data.lock().unwrap().achievements = new_data.achievements.clone();

    update_status(&mut PDM.player_achievements.lock().unwrap()[0], &PDM.player_data.lock().unwrap());
    update_status(&mut PDM.player_achievements.lock().unwrap()[1], &PDM.player_data.lock().unwrap());
    update_status(&mut PDM.player_achievements.lock().unwrap()[2], &PDM.player_data.lock().unwrap());
}

fn update_status(achievements: &mut Achievements, player_data: &PlayerData) {
    for id in 0..achievements.id.len() {
        if player_data.achievements.contains(&achievements.id[id]) {
            achievements.status[id] = true;
        }
    }
}

pub fn create_ui() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new()
                                      .with_resizable(true)
                                      .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1080, 820)))
        );
}

#[derive(PartialEq, Props)]
struct PureAchievement {
    category: String,
    name: Vec<String>,
    id: Vec<i32>,
    status: Vec<bool>
}

fn component_achievements(cx: Scope<PureAchievement>) -> Element {
    let mut page = use_state(cx,|| 0);
    let current_category = use_state(cx, || cx.props.category.clone());

    if cx.props.category != *current_category.get() {
        current_category.set(cx.props.category.clone());
        page.set(0);
    }

    cx.render(rsx!{
        div {class:"table-container",
        table {
            tr{
                th{"{cx.props.category}"}
                th{"Status"}
            }
            for x in (*page.get()*7)..std::cmp::min(*page.get()*7 + 7, cx.props.name.len()) {
                tr{
                    td{"{cx.props.name[x]}"}
                    td{class:"a_{cx.props.status[x]}","{cx.props.status[x]}"}
                }
            }
        }
        button {
            onclick: move |_| {
                if *page.get() > 0 {
                    page -= 1
                }
            },
            "Previous"
        }
        button {
            onclick: move |_| {
                if (*page.get() + 1) * 7 < cx.props.name.len() {
                    page += 1
                }
            },
            "Next"
        }
        }
    })
}

fn app(cx: Scope) -> Element {
    let tab_state = use_state(cx, || 0);

    let data_mutex = Arc::clone(&*PDM);
    let p_data = data_mutex.player_data.lock().unwrap();
    let achievements = data_mutex.player_achievements.lock().unwrap();

    cx.render(rsx! {
        style { include_str!("css/main.css") }
        div { class: "container",
        h1 {class:"title", "FFXIV - Raid Completion Tracker"}

            div { class: "cards",
            div {class:"card",
            h2 {class:"username", "{p_data.name}"}
                p {class:"level", "Lv. {p_data.level}"}
                    p {class:"class", "{p_data.class}"}
            }
            div {class:"card",
            h2 {class:"username", "Datacenter"}
                p {class:"level", "{p_data.datacenter}"}
                    p {class:"class", "{p_data.server}"}
            }
            }

            div {class:"seperator"}

                div {class:"tabs",
                div {class:"tab", onclick: move |_| {
                    tab_state.modify(|_| 0);
                }, "Dungeons"}
                div {class:"tab", onclick: move |_| {
                    tab_state.modify(|_| 1);
                }, "Trials"}
                div {class:"tab", onclick: move |_| {
                    tab_state.modify(|_| 2);
                }, "Raid"}
                }

                component_achievements {
                    category: "Dungeons".to_owned(),
                    name: achievements[*tab_state.get()].name.clone(),
                    id: achievements[*tab_state.get()].id.clone(),
                    status: achievements[*tab_state.get()].status.clone(),
                }

                button {
                    onclick: |_| async move {
                        println!("Quitting the application");
                        std::process::exit(0);
                    },
                    "Exit"
                }
        }
    }) 
}
