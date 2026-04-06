use crate::{load_config, remove_config, save_config};
use dioxus::prelude::*;

#[component]
pub fn Applications() -> Element {
    let mut tables = [0; 4].map(|_| use_signal(|| false));

    let mut keybinds = use_signal(|| {
        let mut k = load_config(Some("niri/config.kdl"), &["binds"]);
        if !k.is_empty() {
            k.remove(0);
            k.pop();
        };
        k
    });

    let mut startups = use_signal(|| {
        let mut s = load_config(Some("niri/config.kdl"), &["spawn-at-startup"]);
        if !s.is_empty() {
            s.pop();
        };
        s
    });

    let mut defaults_ui = use_signal(|| [const { [const { String::new() }; 2] }; 8]);
    let mut make_defaults = move || -> Vec<String> {
        let mut d = load_config(Some("mimeapps.list"), &[""]);
        if !d.is_empty() {
            d.remove(0);
        };
        for default in &d {
            let mut split = default.split('=');
            let key = split.next().unwrap_or("").to_string();
            let value = split.next().unwrap_or("").to_string();
            let value_ui = value.split(".desktop").next().unwrap_or("").to_string();
            match key {
                k if k.starts_with("x-scheme-handler/https") => {
                    defaults_ui.write()[0] = [value, value_ui]
                }
                k if k.starts_with("inode/directory") => defaults_ui.write()[1] = [value, value_ui],
                k if k.starts_with("x-scheme-handler/mailto") => {
                    defaults_ui.write()[2] = [value, value_ui]
                }
                k if k.starts_with("video/mp4") => defaults_ui.write()[3] = [value, value_ui],
                k if k.starts_with("audio/mp3") => defaults_ui.write()[4] = [value, value_ui],
                k if k.starts_with("text/plain") => defaults_ui.write()[5] = [value, value_ui],
                k if k.starts_with("image/png") => defaults_ui.write()[6] = [value, value_ui],
                k if k.starts_with("application/pdf") => defaults_ui.write()[7] = [value, value_ui],
                _ => (),
            }
        }
        d
    };
    let mut defaults = use_signal(make_defaults);

    rsx! {
        div {
            class: "tab",
            h1 { "Applications" }
            div {
                div {
                    p {
                        onclick: move |_| {
                            let state = !tables[0]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[0].set(state);
                        },
                        "GPU Acceleration"
                    }
                }
                div {
                    p {
                        onclick: move |_| {
                            let state = !tables[1]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[1].set(state);
                        },
                        "Keybinds"
                    }
                    div {
                        max_height: if *tables[1].read() { "300px" } else { "0" },
                        overflow_y: if *tables[1].read() { "auto" },
                        visibility: if !*tables[1].read() { "hidden" },

                        form {
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let values = evt.values();
                                let value1 = values.first()
                                        .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                                        .unwrap_or_default();
                                let value2 = values.last()
                                        .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                                        .unwrap_or_default();
                                let command = format!("    {value1} {{ {value2} }}");
                                save_config(Some("niri/config.kdl"), &["binds"], &command);
                                let mut new_list = load_config(Some("niri/config.kdl"), &["binds"]);
                                if !new_list.is_empty() {
                                    new_list.remove(0);
                                    new_list.pop();
                                }
                                keybinds.set(new_list);
                            },
                            table {
                                font_size: "12px", width: "60vw",
                                tr {
                                    th { input { name: "shortcut", placeholder: "Mod+Space" } }
                                    th { input { name: "command", placeholder: "spawn \"rio\";" } }
                                    th { button { padding: "3px", r#type: "submit", "+" } }
                                }
                                for keybind in &keybinds() {{
                                    let first = keybind.split_whitespace().next().unwrap_or("").to_string();
                                    let inside = keybind.split('{').nth(1).and_then(|s| s.split('}').next()).unwrap_or("").to_string();
                                    rsx! {
                                        tr {
                                            td { "{first}" }
                                            td { "{inside}" }
                                            td { onclick: move |_| {
                                                let first_owned = first.clone();
                                                spawn(async move {
                                                    let confirmed = rfd::AsyncMessageDialog::new()
                                                        .set_level(rfd::MessageLevel::Info)
                                                        .set_title("Confirm Deletion")
                                                        .set_description(format!("Delete keybind: {}?", first_owned))
                                                        .set_buttons(rfd::MessageButtons::YesNo)
                                                        .show()
                                                        .await;
                                                    if confirmed == rfd::MessageDialogResult::Yes {
                                                        remove_config(Some("niri/config.kdl"), &[&first_owned]);
                                                        let mut new_list = load_config(Some("niri/config.kdl"), &["binds"]);
                                                        if !new_list.is_empty() {
                                                            new_list.remove(0);
                                                            new_list.pop();
                                                        }
                                                        keybinds.set(new_list);
                                                    }
                                                });
                                            }, text_align: "center",  width: "6px", "X" }
                                        }
                                    }

                                }}
                            }
                        }
                    }
                }
                div {
                    p {
                        onclick: move |_| {
                            let state = !tables[2]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[2].set(state);
                        },
                        "Autostarts"
                    }
                    div {
                        max_height: if *tables[2].read() { "300px" } else { "0" },
                        overflow_y: if *tables[2].read() { "auto" },
                        visibility: if !*tables[2].read() { "hidden" },
                        form {
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let values = evt.values();
                                let value = values.first()
                                        .and_then(|(_, v)| if let FormValue::Text(s) = v { Some(s.clone()) } else { None })
                                        .unwrap_or_default();
                                let command = format!("spawn-at-startup \"{value}\"");
                                save_config(Some("niri/config.kdl"), &["// startups"], &command);
                                let mut new_list = load_config(Some("niri/config.kdl"), &["spawn-at-startup"]);
                                if !new_list.is_empty() {
                                    new_list.pop();
                                }
                                startups.set(new_list);
                            },
                            table {
                                font_size: "20px",
                                tr {
                                    th { input { name: "command", placeholder: "xwayland-satellite" } }
                                    th { button { padding: "3px", r#type: "submit", "+" } }
                                }
                                for startup in &startups() {{
                                    let inside = startup.split('"').nth(1).and_then(|s| s.split('"').next()).unwrap_or("").to_string();
                                    rsx! {
                                        tr {
                                            td { "{inside}" }
                                            td { onclick: move |_| {
                                                let inside_owned = inside.clone();
                                                spawn(async move {
                                                    let confirmed = rfd::AsyncMessageDialog::new()
                                                        .set_level(rfd::MessageLevel::Info)
                                                        .set_title("Confirm Deletion")
                                                        .set_description(format!("Delete startup: {}?", inside_owned))
                                                        .set_buttons(rfd::MessageButtons::YesNo)
                                                        .show()
                                                        .await;
                                                    if confirmed == rfd::MessageDialogResult::Yes {
                                                        remove_config(Some("niri/config.kdl"), &[&inside_owned]);
                                                        let mut new_list = load_config(Some("niri/config.kdl"), &["spawn-at-startup"]);
                                                        if !new_list.is_empty() {
                                                            new_list.pop();
                                                        }
                                                        startups.set(new_list);
                                                    }
                                                });
                                            }, text_align: "center",  width: "6px", "X" }
                                        }
                                    }
                                }}
                            }
                        }
                    }
                }
                div {
                    p {
                        onclick: move |_| {
                            let state = !tables[3]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[3].set(state);
                        },
                        "Defaults"
                    }
                    div {
                        max_height: if *tables[3].read() { "400px" } else { "0" },
                        overflow_y: if *tables[3].read() { "auto" },
                        visibility: if !*tables[3].read() { "hidden" },

                        form {
                            onsubmit: move |evt| {
                                let values = evt.values();
                                spawn(async move {
                                    let confirmed = rfd::AsyncMessageDialog::new()
                                        .set_level(rfd::MessageLevel::Info)
                                        .set_title("Confirm Change")
                                        .set_description("Replace the defaults?")
                                        .set_buttons(rfd::MessageButtons::YesNo)
                                        .show()
                                        .await;
                                    if confirmed == rfd::MessageDialogResult::Yes {
                                        let mut change_default = move |link: &str, value: &String| {
                                            defaults()
                                                .iter()
                                                .filter(|w: &&String| (**w).contains(link))
                                                .for_each(|w: &String| {
                                                    let key = w.split("=").next().unwrap_or("").to_string() + "=";
                                                    let change = format!("{key}{value}.desktop;");
                                                    save_config(Some("mimeapps.list"), &[&key], &change);
                                                    remove_config(Some("mimeapps.list"), &[&key]);
                                                    defaults.set(make_defaults());
                                                });
                                        };
                                        for (key, form) in values {
                                            if let FormValue::Text(value) = form {
                                                if value.is_empty() { continue; }
                                                match &*key {
                                                    "browser" => change_default("x-scheme-handler/http", &value),
                                                    "manager" => change_default("inode/directory", &value),
                                                    "mail" => change_default("x-scheme-handler/mailto", &value),
                                                    "video" => change_default("video/", &value),
                                                    "audio" => change_default("audio/", &value),
                                                    "text" => change_default("text/", &value),
                                                    "image" => change_default("image/", &value),
                                                    "pdf" => change_default("application/pdf", &value),
                                                    _ => continue,
                                                }
                                            }
                                        }
                                    }
                                });
                            },
                            table {
                                font_size: "17px", width: "60vw",
                                tr { td { "Browser" } td { "{defaults_ui()[0][1]}" } td { input { name: "browser" } } }
                                tr { td { "File Manager" } td { "{defaults_ui()[1][1]}" } td { input { name: "manager" } } }
                                tr { td { "Mail" } td { "{defaults_ui()[2][1]}" } td { input { name: "mail" } } }
                                tr { td { "Video" } td { "{defaults_ui()[3][1]}" } td { input { name: "video" } } }
                                tr { td { "Audio" } td { "{defaults_ui()[4][1]}" } td { input { name: "audio" } } }
                                tr { td { "Text" } td { "{defaults_ui()[5][1]}" } td { input { name: "text" } } }
                                tr { td { "Image" } td { "{defaults_ui()[6][1]}" } td { input { name: "image" } } }
                                tr { td { "PDF" } td { "{defaults_ui()[7][1]}" } td { input { name: "pdf" } } }
                            }
                            button { padding: "3px", width: "60vw", r#type: "submit", "Save" }
                        }
                    }
                }
            }
        }
    }
}
