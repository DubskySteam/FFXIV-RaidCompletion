use dioxus::prelude::*;
use dioxus_desktop::*;

pub fn create_ui() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hi, there!"
        }
    })
}
