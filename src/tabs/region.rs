use dioxus::prelude::*;

#[component]
pub fn Region() -> Element {
    rsx! {
        div {
            padding: "20px",
            h1 { "Region" }
        }
    }
}
