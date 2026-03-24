use dioxus::prelude::*;

#[component]
pub fn Update() -> Element {
    let mut is_updating = use_signal(|| false);
    let mut update_msg = use_signal(|| "");
    rsx! {
        div {
            width: "100vw", padding: "20px",
            h1 { "Update" }
            div {
                align_items: "center",
                button {
                    disabled: is_updating(),
                    onclick: move |_| {
                        is_updating.set(true);
                        update_msg.set("Updating...");

                        spawn(async move {
                            let mut child = tokio::process::Command::new("sudo")
                                .args(["pacman", "-Syu", "--noconfirm"])
                                .spawn()
                                .expect("failed to spawn");

                            let done = child.wait().await;

                            is_updating.set(false);
                            if done.is_ok() {
                                update_msg.set("Up to date");
                            } else {
                                update_msg.set("Something Went Wrong. Try Again");
                            }

                        });
                    },
                    float: "left", margin: "7px",
                    "Check For Updates"
                }
                p { color: "#555555", font_size: "20px", float: "left", "{update_msg()}"}
            }

        }
    }
}
