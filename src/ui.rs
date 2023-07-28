use dioxus::prelude::*;
use dioxus_desktop::*;
use std::sync::{Mutex, Arc};
use crate::content::Achievements;
use crate::PlayerData::PlayerData;

struct PlayerDataMutex {
    player_data: Mutex<PlayerData>,
    player_achievements: Mutex<[Achievements; 3]>
}

impl PlayerDataMutex {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            player_data: Mutex::new(PlayerData::new()),
            player_achievements: Mutex::new([
                Achievements::new(),
                Achievements::new(),
                Achievements::new()
            ])
        })
    }

    fn update(&self, new_data: &PlayerData) {
        if self.player_achievements.lock().unwrap()[0].status.len() <= 1 {
            match Achievements::read_data("dungeons") {
                Ok(result) => self.player_achievements.lock().unwrap()[0] = result,
                Err(e) => eprintln!("Error >> {}", e),
            }
            match Achievements::read_data("trials") {
                Ok(result) => self.player_achievements.lock().unwrap()[1] = result,
                Err(e) => eprintln!("Error >> {}", e),
            }
            match Achievements::read_data("raids") {
                Ok(result) => self.player_achievements.lock().unwrap()[2] = result,
                Err(e) => eprintln!("Error >> {}", e),
            }
        }
        self.player_data.name = new_data.name.clone();
        self.player_data.level = new_data.level;
        self.player_data.class = new_data.class.clone();
        self.player_data.server = new_data.server.clone();
        self.player_data.datacenter = new_data.datacenter.clone();
        self.player_data.achievements = new_data.achievements.clone();

        update_status(&mut self.p_achievements_dun, &self.player_data);
        update_status(&mut self.p_achievements_trial, &state.player_data);
        update_status(&mut self.p_achievements_raid, &state.player_data);
    }
}

static mut PDM: Arc<PlayerDataMutex> = PlayerDataMutex::new();

pub fn create_ui() {
    let mut pdm = PlayerDataMutex::new();

    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new()
                                      .with_resizable(true)
                                      .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1080, 820)))
        );
}

fn update_status(achievements: &mut Achievements, player_data: &PlayerData) {
    for id in 0..achievements.id.len() {
        if player_data.achievements.contains(&achievements.id[id]) {
            achievements.status[id] = true;
        }
    }
}
fn c_dungeon(cx: Scope) -> Element {
    let mut page = use_self(cx,|| 0);
    unsafe {
        cx.render(rsx!{
            div {class:"table-container",
            table {
                tr{
                    th{"Dungeon"}
                    th{"Status"}
                }
                for x in (*page.get()*7)..std::cmp::min(*page.get()*7 + 7, P_ACHIEVEMENTS_DUN.name.len()) {
                    tr{
                        td{"{P_ACHIEVEMENTS_DUN.name[x]}"}
                        td{class:"a_{P_ACHIEVEMENTS_DUN.status[x]}","{P_ACHIEVEMENTS_DUN.status[x]}"}
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
                    if *page.get() < 1 {
                        page += 1
                    }
                },
                "Next"
            }
            }
        })
    }
}
fn c_trial(cx: Scope) -> Element {
    let mut page = use_self(cx,|| 0);
    unsafe {
        cx.render(rsx!{
            div {class:"table-container",
            table {
                tr{
                    th{"Trial"}
                    th{"Status"}
                }
                for x in (*page.get()*7)..std::cmp::min(*page.get()*7 + 7, P_ACHIEVEMENTS_TRIAL.name.len()) {
                    tr{
                        td{"{P_ACHIEVEMENTS_TRIAL.name[x]}"}
                        td{class:"a_{P_ACHIEVEMENTS_TRIAL.status[x]}","{P_ACHIEVEMENTS_TRIAL.status[x]}"}
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
                    if *page.get() < 4 {
                        page += 1
                    }
                },
                "Next"
            }
            }
        })
    }
}
fn c_raid(cx: Scope) -> Element {
    let mut page = use_self(cx,|| 0);
    unsafe {
        cx.render(rsx!{
            div {class:"table-container",
            table {
                tr{
                    th{"Raid"}
                    th{"Status"}
                }
                for x in (*page.get()*7)..std::cmp::min(*page.get()*7 + 7, P_ACHIEVEMENTS_RAID.name.len()) {
                    tr{
                        td{"{P_ACHIEVEMENTS_RAID.name[x]}"}
                        td{class:"a_{P_ACHIEVEMENTS_RAID.status[x]}","{P_ACHIEVEMENTS_RAID.status[x]}"}
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
                    if *page.get() < 2 {
                        page += 1
                    }
                },
                "Next"
            }
           }
        })
    }
}

fn app(cx: Scope) -> Element {
    let tab_self = use_state(cx, || 0);
    unsafe {
        if *tab_self == 0 {
            cx.render(rsx! {
                style { include_str!("css/main.css") }
                div { class: "container",
                h1 {class:"title", "FFXIV - Raid Completion Tracker"}

                    div { class: "cards",
                    div {class:"card",
                    h2 {class:"username", "{P_DATA.name}"}
                        p {class:"level", "Lv. {P_DATA.level}"}
                            p {class:"class", "{P_DATA.class}"}
                    }
                    div {class:"card",
                    h2 {class:"username", "Datacenter"}
                        p {class:"level", "{P_DATA.datacenter}"}
                            p {class:"class", "{P_DATA.server}"}
                    }
                    }

                    div {class:"seperator"}

                        div {class:"tabs",
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 0);
                        }, "Dungeons"}
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 1);
                        }, "Trials"}
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 2);
                        }, "Raid"}
                        }

                        c_dungeon{}

                        button {
                            onclick: |_| async move {
                                println!("Quitting the application");
                                std::process::exit(0);
                            },
                            "Exit"
                        }
                }
            })
        } else if *tab_self == 1 {
            cx.render(rsx! {
                style { include_str!("css/main.css") }
                div { class: "container",
                h1 {class:"title", "FFXIV - Raid Completion Tracker"}

                    div { class: "cards",
                    div {class:"card",
                    h2 {class:"username", "{P_DATA.name}"}
                        p {class:"level", "Lv. {P_DATA.level}"}
                            p {class:"class", "{P_DATA.class}"}
                    }
                    div {class:"card",
                    h2 {class:"username", "Datacenter"}
                        p {class:"level", "{P_DATA.datacenter}"}
                            p {class:"class", "{P_DATA.server}"}
                    }
                    }

                    div {class:"seperator"}


                        div {class:"tabs",
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 0);
                        }, "Dungeons"}
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 1);
                        }, "Trials"}
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 2);
                        }, "Raid"}
                        }

                            c_trial{}

                        button {
                            onclick: |_| async move {
                                println!("Quitting the application");
                                std::process::exit(0);
                            },
                            "Exit"
                        }
                }
            })

        } else {
            cx.render(rsx! {
                style { include_str!("css/main.css") }
                div { class: "container",
                h1 {class:"title", "FFXIV - Raid Completion Tracker"}

                    div { class: "cards",
                    div {class:"card",
                    h2 {class:"username", "{P_DATA.name}"}
                        p {class:"level", "Lv. {P_DATA.level}"}
                            p {class:"class", "{P_DATA.class}"}
                    }
                    div {class:"card",
                    h2 {class:"username", "Datacenter"}
                        p {class:"level", "{P_DATA.datacenter}"}
                            p {class:"class", "{P_DATA.server}"}
                    }
                    }

                    div {class:"seperator"}


                        div {class:"tabs",
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 0);
                        }, "Dungeons"}
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 1);
                        }, "Trials"}
                        div {class:"tab", onclick: move |_| {
                            tab_self.modify(|_| 2);
                        }, "Raid"}
                        }

                            c_raid{}

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
    }
}
