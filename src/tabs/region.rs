use crate::{load_config, save_config};
use dioxus::prelude::*;

#[component]
pub fn Region() -> Element {
    let mut current_timezone =
        use_signal(|| load_config("timezone").unwrap_or_else(|| "America/New_York".to_string()));
    let timezones = get_timezones();
    let mut current_language =
        use_signal(|| load_config("language").unwrap_or_else(|| "en_US.UTF-8".to_string()));
    let languages = get_languages();
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
                            if timezone == current_timezone() {
                                option { value: "{timezone}", selected: true, "{timezone}" }
                            } else {
                                option { value: "{timezone}", "{timezone}" }
                            }
                        }
                    }
                }
                div {
                    height: "70px", width: "60vw", align_items: "center",
                    p { float: "left", margin: "20px", "System Language" },
                    select {
                        onchange: move |evt: FormEvent| {
                            current_language.set(evt.value());
                            let var = evt.value();
                            let lang_id = var.split_whitespace().next().unwrap_or("en_US.UTF-8");

                            std::process::Command::new("sh")
                                .args(["-c", &format!("grep -qxF '{var}' /etc/locale.gen || echo '{var}' | sudo tee -a /etc/locale.gen")])
                                .status().ok();

                            std::process::Command::new("sudo").arg("locale-gen").status().ok();
                            std::process::Command::new("sudo").args(["localectl", "set-locale", &format!("LANG={lang_id}")]).status().ok();
                            save_config("language", &evt.value());
                        },
                        float: "right", margin: "20px", width: "150px",

                        for language in languages {
                            if language == current_language() {
                                option { value: "{language}", selected: true, "{language}" }
                            } else {
                                option { value: "{language}", "{language}" }
                            }
                        }
                    }
                }
                div {
                    height: "70px", width: "60vw", align_items: "center",
                    p { float: "left", margin: "20px", "Keyboard Layout" },
                    select {
                        float: "right", margin: "20px", width: "150px",
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
