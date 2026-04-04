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

    rsx! {
        div {
            class: "tab",
            h1 { "Applications" }
            div {
                div {
                    p { "GPU Acceleration" }
                }
                div {
                    p {
                        onclick: move |_| tables[1].toggle(),
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
                        onclick: move |_| tables[2].toggle(),
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
                    p { "Defaults" }
                }
            }
        }
    }
}
