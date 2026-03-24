use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/main.css");

mod tabs;

fn main() {
    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(true);

    let cfg = Config::new()
        .with_window(window)
        .with_disable_drag_drop_handler(true);

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut tab = use_signal(|| 0);

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        main {
            display: "flex",
            div {
                id: "sidebar",
                width: "300px", height: "100vh",
                background_color: "#1f2126",
                h2 { font_weight: "bold", padding: "20px",  "Haremal Controller" },
                button { onclick: move |_| tab.set(0), class: "tab_button", background_color: if tab() == 0 { "#3f4146" },  "Update" }
                button { onclick: move |_| tab.set(1), class: "tab_button", background_color: if tab() == 1 { "#3f4146" },  "Region" }
                button { onclick: move |_| tab.set(2), class: "tab_button", background_color: if tab() == 2 { "#3f4146" },  "Applications" }
                button { onclick: move |_| tab.set(3), class: "tab_button", background_color: if tab() == 3 { "#3f4146" },  "Devices" }
                button { onclick: move |_| tab.set(4), class: "tab_button", background_color: if tab() == 4 { "#3f4146" },  "Appearance" }
                button { onclick: move |_| tab.set(5), class: "tab_button", background_color: if tab() == 5 { "#3f4146" },  "Desktop" }
            }
            match tab() {
                1 => tabs::region::Region(),
                2 => tabs::applications::Applications(),
                3 => tabs::devices::Devices(),
                4 => tabs::appearance::Appearance(),
                5 => tabs::desktop::Desktop(),
                _ => tabs::update::Update(),
            }
        }
    }
}
