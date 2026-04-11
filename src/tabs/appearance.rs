use dioxus::prelude::*;

#[component]
pub fn Appearance() -> Element {
    rsx! {
        div {
            class: "tab",
            h1 { "Appearance" }
            div {
                "TODO: FIX APPLICATIONS"
                "- TAB_BUTTONS"
                "- INPUTS (BORDER COLORS, PLACE HOLDERS)"
                "- DEFAULTS LOOK BUG"
            }
        }
    }
}
