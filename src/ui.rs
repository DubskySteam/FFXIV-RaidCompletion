#![allow(unused_variables)]
use dioxus::{prelude::*, html::{ul, button, br, h1, table, th, tr}};
use dioxus_desktop::*;

use crate::content::{self, Achievements};
use crate::player::PlayerData;

static mut P_DATA: PlayerData = PlayerData {
            name: String::new(),
            level: 0,
            class: String::new(), datacenter: String::new(),
            server: String::new(),
            achievements: Vec::new()
        };

static mut P_ACHIEVEMENTS_DUN: Achievements = Achievements {
    name: Vec::new(),
    id: Vec::new(),
    status: Vec::new()
};
static mut P_ACHIEVEMENTS_TRIAL: Achievements = Achievements {
    name: Vec::new(),
    id: Vec::new(),
    status: Vec::new()
};
static mut P_ACHIEVEMENTS_RAID: Achievements = Achievements {
    name: Vec::new(),
    id: Vec::new(),
    status: Vec::new()
};

pub fn create_ui() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new()
                                      .with_resizable(true)
                                      .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1080, 820)))
        );
}

pub fn update(new_data: &PlayerData) {
    unsafe {
        if P_ACHIEVEMENTS_DUN.status.len() <= 1 {
            match Achievements::read_data("dungeons") {
                Ok(result) => {P_ACHIEVEMENTS_DUN = result},
                Err(e) => {eprintln!("Error >> {}", e)}
            }
            match Achievements::read_data("trials") {
                Ok(result) => {P_ACHIEVEMENTS_TRIAL = result},
                Err(e) => {eprintln!("Error >> {}", e)}
            }
            match Achievements::read_data("raids") {
                Ok(result) => {P_ACHIEVEMENTS_RAID = result},
                Err(e) => {eprintln!("Error >> {}", e)}
            }
        }
        P_DATA.name = new_data.name.clone();
        P_DATA.level = new_data.level.clone();
        P_DATA.class = new_data.class.clone();
        P_DATA.server = new_data.server.clone();
        P_DATA.datacenter = new_data.datacenter.clone();
        P_DATA.achievements = new_data.achievements.clone();
        //PMD
        println!("UPDATED DATA");
        println!("Achievement Array: {:?}",
                 P_DATA.achievements
                );
        for id in 0..P_ACHIEVEMENTS_DUN.id.len() {
            if P_DATA.achievements.contains(&P_ACHIEVEMENTS_DUN.id[id]) {
                P_ACHIEVEMENTS_DUN.status[id] = true;
            }
        }
        for id in 0..P_ACHIEVEMENTS_TRIAL.id.len() {
            if P_DATA.achievements.contains(&P_ACHIEVEMENTS_TRIAL.id[id]) {
               P_ACHIEVEMENTS_TRIAL.status[id] = true;
            }
        }
        for id in 0..P_ACHIEVEMENTS_RAID.id.len() {
            if P_DATA.achievements.contains(&P_ACHIEVEMENTS_RAID.id[id]) {
               P_ACHIEVEMENTS_RAID.status[id] = true;
            }
        }
    }
}

fn c_dungeon(cx: Scope) -> Element {
    let mut page = use_state(cx,|| 0);
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
    let mut page = use_state(cx,|| 0);
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
    let mut page = use_state(cx,|| 0);
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

fn App(cx: Scope) -> Element {
    let tab_state = use_state(cx, || 0);
    unsafe {
        if *tab_state == 0 {
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
                            tab_state.modify(|val| 0);
                        }, "Dungeons"}
                        div {class:"tab", onclick: move |_| {
                            tab_state.modify(|val| 1);
                        }, "Trials"}
                        div {class:"tab", onclick: move |_| {
                            tab_state.modify(|val| 2);
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
        } else if *tab_state == 1 {
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
                            tab_state.modify(|val| 0);
                        }, "Dungeons"}
                        div {class:"tab", onclick: move |_| {
                            tab_state.modify(|val| 1);
                        }, "Trials"}
                        div {class:"tab", onclick: move |_| {
                            tab_state.modify(|val| 2);
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
                            tab_state.modify(|val| 0);
                        }, "Dungeons"}
                        div {class:"tab", onclick: move |_| {
                            tab_state.modify(|val| 1);
                        }, "Trials"}
                        div {class:"tab", onclick: move |_| {
                            tab_state.modify(|val| 2);
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
