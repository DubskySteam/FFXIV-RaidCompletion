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
        println!("Char: {}\nLevel: {}\nClass: {}\nDC: {}\nServer: {}\nAchievements: {:?}",
                 P_DATA.name,
                 P_DATA.level,
                 P_DATA.class,
                 P_DATA.datacenter,
                 P_DATA.server,
                 P_DATA.achievements
                );
    }
}

fn App(cx: Scope) -> Element {
    unsafe {
        cx.render(rsx! {
            div {
                ul {
                    li {"{P_DATA.name}"}
                    li {"{P_DATA.class}"}
                    li {"{P_DATA.level}"}
                    li {"{P_DATA.server}"}
                }
                button {
                    onclick: move |event| println!("Trying to refresh the data...")
                }
            }
        })
    }
}
