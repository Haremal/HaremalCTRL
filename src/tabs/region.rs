use crate::{edit_config, load_config, save_config};
use dioxus::prelude::*;

#[component]
pub fn Region() -> Element {
    let mut current_timezone =
        use_signal(|| load_config("timezone").unwrap_or_else(|| "America/New_York".to_string()));
    let timezones = get_timezones();

    let mut current_language =
        use_signal(|| load_config("language").unwrap_or_else(|| "en_US.UTF-8".to_string()));
    let languages = get_languages();
    let mut seen_languages = std::collections::HashSet::new();
    let mut languages_disabled = use_signal(|| false);

    let mut current_layout =
        use_signal(|| load_config("layout").unwrap_or_else(|| "us".to_string()));
    let layouts = get_layouts();

    rsx! {
        div {
            font_size: "150%", padding: "20px",
            h1 { "Region" }
            div {
                div {
                    height: "70px", width: "60vw", align_items: "center",
                    p { float: "left", margin: "20px", "Timezone" },
                    select {
                        onchange: move |evt: FormEvent| {
                            current_timezone.set(evt.value());
                            std::process::Command::new("sudo")
                                .args(["timedatectl", "set-timezone", &evt.value()])
                                .spawn()
                                .expect("Failed to set timezone").wait().unwrap();
                            save_config("timezone", &evt.value());
                        },
                        float: "right", margin: "20px", width: "150px",

                        for timezone in timezones {
                            option { value: "{timezone}", selected: timezone == current_timezone(), "{timezone}" }
                        }
                    }
                }
                div {
                    height: "70px", width: "60vw", align_items: "center",
                    p { float: "left", margin: "20px", "System Language" },
                    select {
                        disabled: languages_disabled(),
                        onchange: move |evt: FormEvent| {
                            languages_disabled.set(true);
                            let val = evt.value();
                            let lang_id = val.split_whitespace().next().unwrap_or("en_US.UTF-8").to_string();
                            let var = val.clone();
                            spawn(async move {
                                let _ = tokio::process::Command::new("sh").args(["-c", &format!("grep -qxF '{var}' /etc/locale.gen || echo '{var}' | sudo tee -a /etc/locale.gen")]).status().await;
                                let _ = tokio::process::Command::new("sudo").arg("locale-gen").status().await;
                                let _ = tokio::process::Command::new("sudo").args(["localectl", "set-locale", &format!("LANG={lang_id}")]).status().await;
                                languages_disabled.set(false);

                                use_context::<Signal<bool>>().set(true);
                            });

                            current_language.set(evt.value());
                            save_config("language", &evt.value());

                        },
                        float: "right", margin: "20px", width: "150px",

                        {
                            languages.iter().filter_map(|language| {
                                if language.contains('@') || !language.contains(".UTF-8") || language == "C.UTF-8" { return None; }
                                let lang_id = language.split('_').next().unwrap_or(language);
                                if !seen_languages.insert(lang_id.to_string()) { return None; }
                                let pretty_name = if lang_id.len() == 2 || lang_id.len() == 3 {
                                    locale_codes::language::lookup(lang_id)
                                        .map(|l| l.reference_name.to_string())
                                        .unwrap_or_else(|| lang_id.to_uppercase())
                                } else {
                                    return None;
                                };
                                Some(rsx! {
                                    option {
                                        value: "{language}",
                                        selected: *language == current_language(),
                                        "{pretty_name}"
                                    }
                                })
                            })
                        }
                    }
                }
                div {
                    height: "70px", width: "60vw", align_items: "center",
                    p { float: "left", margin: "20px", "Keyboard Layout" },
                    select {
                        onchange: move |evt: FormEvent| {
                            current_layout.set(evt.value());
                            save_config("layout", &evt.value());
                            edit_config("niri/config.kdl", &["input", "keyboard", "xkb", "layout"], &evt.value());
                        },
                        float: "right", margin: "20px", width: "150px",

                        for layout in layouts {
                            option { value: "{layout}", selected: layout  == current_layout(), "{layout}" }
                        }
                    }
                }
            }
        }
    }
}

fn get_timezones() -> Vec<String> {
    let output = std::process::Command::new("timedatectl")
        .arg("list-timezones")
        .output()
        .expect("Failed to fetch timezones");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn get_languages() -> Vec<String> {
    let output = std::process::Command::new("cat")
        .arg("/usr/share/i18n/SUPPORTED")
        .output()
        .expect("Failed to fetch languages");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn get_layouts() -> Vec<String> {
    let output = std::process::Command::new("localectl")
        .arg("list-x11-keymap-layouts")
        .output()
        .expect("Failed to fetch layouts");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}
