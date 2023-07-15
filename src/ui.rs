use dioxus::prelude::*;
use dioxus_desktop::*;

pub fn create_ui() {
    dioxus_desktop::launch_cfg(
        App,
        Config::default().with_window(WindowBuilder::new()
                                      .with_resizable(false)
                                      .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1080, 720)))
        );
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hi, there!"
        }
    })
}
