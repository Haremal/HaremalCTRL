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
                1 => rsx! { tabs::region::Region {} },
                2 => rsx! { tabs::applications::Applications {} },
                3 => rsx! { tabs::devices::Devices {} },
                4 => rsx! { tabs::appearance::Appearance {} },
                5 => rsx! { tabs::desktop::Desktop {} },
                _ => rsx! { tabs::update::Update {} },
            }
        }
    }
}

pub fn save_config(key: &str, value: &str) {
    let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
    let mut path = std::path::PathBuf::from(base);

    path.push("haremal-ctrl");
    if let Err(e) = std::fs::create_dir_all(&path) {
        eprintln!("Failed to create config directory: {}", e);
        return;
    }

    path.push("config.toml");
    let content = std::fs::read_to_string(&path).unwrap_or_default();

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut found = false;

    for line in lines.iter_mut() {
        if line.trim().starts_with(key) && line.contains('=') {
            *line = format!("{} = \"{}\"", key, value);
            found = true;
            break;
        }
    }

    if !found {
        lines.push(format!("{} = \"{}\"", key, value));
    }

    let new_content = lines.join("\n");
    if let Err(e) = std::fs::write(&path, new_content) {
        eprintln!("Error saving config: {}", e);
    }
}

pub fn load_config(key: &str) -> Option<String> {
    let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
    let path = std::path::PathBuf::from(base).join("haremal-ctrl/config.toml");

    let content = std::fs::read_to_string(path).ok()?;

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    for line in lines.iter_mut() {
        if line.trim().starts_with(key) {
            let parts: Vec<&str> = line.split('=').collect();
            return Some(parts.get(1)?.trim().replace('"', ""));
        }
    }
    None
}
