use crate::utils::chart::Chart;
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx!({ Chart() })
}
