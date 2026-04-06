use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use std::path::Path;
const CSS: Asset = asset!("/assets/main.css");

mod tabs;

fn main() {
    std::fs::write("debug.log", "App started\n").ok();
    let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
    let path = std::path::PathBuf::from(&base).join("haremal-ctrl");
    if let Err(e) = std::fs::create_dir_all(&path) {
        eprintln!("Failed to create config directory: {}", e);
        return;
    }
    let mimeapps_path = std::path::PathBuf::from(base).join("mimeapps.list");
    if !Path::new(&mimeapps_path).exists() {
        let mut mimeapps = load_config(Some("/usr/share/applications/mimeinfo.cache"), &[""]);
        mimeapps.retain(|item| {
            let m = &item.to_string();
            m.starts_with("inode/directory")
                || m.starts_with("text/")
                || m.starts_with("image/")
                || m.starts_with("video/")
                || m.starts_with("audio/")
                || m.starts_with("application/pdf")
                || m.starts_with("x-scheme-handler/https")
                || m.starts_with("x-scheme-handler/http")
                || m.starts_with("x-scheme-handler/mailto")
        });
        for mimeapp in &mimeapps {
            save_config(Some("mimeapps.list"), &[""], mimeapp);
        }
        save_config(Some("mimeapps.list"), &[""], "[Default Applications]");
    }

    let window = WindowBuilder::new()
        .with_title("HaremalCTRL")
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(true);

    let cfg = Config::new()
        .with_window(window)
        .with_disable_context_menu(true)
        .with_disable_drag_drop_handler(true);

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut tab = use_signal(|| 0);
    let major_changes = use_context_provider(|| Signal::new(false));
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        main {
            display: "flex",
            div {
                border_radius: "16px 0px 0px 16px",
                width: "300px", height: "100vh",
                background_color: "#1f2126",
                h2 { font_size: "200%", font_weight: "bold", padding_left: "20px", "HaremalCTRL Settings" },
                button { onclick: move |_| tab.set(0), class: "tab_button", background_color: if tab() == 0 { "#3f4146" },  "Update" }
                button { onclick: move |_| tab.set(1), class: "tab_button", background_color: if tab() == 1 { "#3f4146" },  "Region" }
                button { onclick: move |_| tab.set(2), class: "tab_button", background_color: if tab() == 2 { "#3f4146" },  "Applications" }
                button { onclick: move |_| tab.set(3), class: "tab_button", background_color: if tab() == 3 { "#3f4146" },  "Devices" }
                button { onclick: move |_| tab.set(4), class: "tab_button", background_color: if tab() == 4 { "#3f4146" },  "Appearance" }
                button { onclick: move |_| tab.set(5), class: "tab_button", background_color: if tab() == 5 { "#3f4146" },  "Desktop" }
                h3 { visibility: if !major_changes() { "hidden" }, color: "orange", opacity: "50%", font_weight: "200", position: "absolute", bottom: "60px", left: "20px", width: "300px", "Some changes might require a restart to work properly"}
                button { visibility: if !major_changes() { "hidden" }, onclick: move |_| { std::process::Command::new("reboot").status().ok(); }, position: "absolute", bottom: "20px", left: "20px", color: "orange", opacity: "50%", "REBOOT NOW" }
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

pub fn load_config(file: Option<&str>, keys: &[&str]) -> Vec<String> {
    let filename = file.unwrap_or("haremal-ctrl/config.toml");
    let path = if !filename.starts_with("/") {
        let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
        std::path::PathBuf::from(base).join(filename)
    } else {
        std::path::PathBuf::from(filename)
    };
    let content = std::fs::read_to_string(path).ok().unwrap_or_default();
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut passed = 0;
    let mut list = Vec::new();
    let mut collecting = false;
    for line in lines.into_iter() {
        let trimmed = line.trim();
        if !collecting && line.trim().starts_with(keys[passed]) {
            passed += 1;
            if passed >= keys.len() {
                collecting = true;
            }
        }

        if collecting {
            passed += trimmed.matches('{').count();
            passed -= trimmed.matches('}').count();
            list.push(line.to_string());
            if passed < keys.len() {
                break;
            }
        }
    }
    list
}

pub fn save_config(file: Option<&str>, keys: &[&str], value: &str) {
    let filename = file.unwrap_or("haremal-ctrl/config.toml");
    let path = if !filename.starts_with("/") {
        let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
        std::path::PathBuf::from(base).join(filename)
    } else {
        std::path::PathBuf::from(filename)
    };
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    if lines.is_empty() {
        lines.push(String::new());
    }

    let mut passed = 0;
    let mut starting_line = 0;
    for (i, line) in lines.iter_mut().enumerate() {
        if line.trim().starts_with(keys[passed]) {
            starting_line = i;
            passed += 1;
            if passed >= keys.len() {
                if let (Some(start), Some(end)) = (line.find('"'), line.rfind('"')) {
                    if start != end {
                        line.replace_range(start + 1..end, value);
                    }
                } else {
                    lines.insert(starting_line + 1, String::from(value));
                }
                break;
            }
        }
    }

    while passed < keys.len() {
        if passed == keys.len() - 1 {
            lines.insert(starting_line + 1, format!("{} \"{}\"", keys[passed], value));
        } else {
            lines.insert(starting_line + 1, format!("{} {{", keys[passed]));
            lines.insert(starting_line + 2, "}".to_string());
        }
        starting_line += 1;
        passed += 1;
    }

    let new_content = lines.join("\n");
    if let Err(e) = std::fs::write(&path, new_content) {
        eprintln!("Error saving config: {}", e);
    }
}

pub fn remove_config(file: Option<&str>, keys: &[&str]) {
    let filename = file.unwrap_or("haremal-ctrl/config.toml");
    let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
    let path = std::path::PathBuf::from(base).join(filename);
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut passed = 0;
    let mut to_remove = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().contains(keys[passed]) {
            passed += 1;
            if passed >= keys.len() {
                to_remove = Some(i);
                break;
            }
        }
    }
    if let Some(i) = to_remove {
        lines.remove(i);
    }
    let new_content = lines.join("\n");
    if let Err(e) = std::fs::write(&path, new_content) {
        eprintln!("Error saving config: {}", e);
    }
}
