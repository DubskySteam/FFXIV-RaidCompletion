#![allow(unused_variables)]
use dioxus::{prelude::*, html::{ul, button}};
use dioxus_desktop::*;

use crate::player::PlayerData;

static mut P_DATA: PlayerData = PlayerData {
            name: String::new(),
            level: 0,
            class: String::new(),
            datacenter: String::new(),
            server: String::new(),
            achievements: Vec::new()
        };

pub fn create_ui() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new()
                                      .with_resizable(false)
                                      .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1080, 720)))
        );
}

pub fn update(new_data: &PlayerData) {
    unsafe {
        P_DATA.name = new_data.name.clone();
        P_DATA.level = new_data.level.clone();
        P_DATA.class = new_data.class.clone();
        P_DATA.server = new_data.server.clone();
        P_DATA.datacenter = new_data.datacenter.clone();
        P_DATA.achievements = new_data.achievements.clone();
        println!("UPDATED DATA");
        println!("Achievement Array: {:?}",
                 P_DATA.achievements
                );
    }
}

fn App(cx: Scope) -> Element {
    unsafe {
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
                    div {class:"tab", "Dungeons"}
                    div {class:"tab", "Trial"}
                    div {class:"tab", "Raid"}
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
}
