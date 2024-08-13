use crate::utils::chart::Chart;
use dioxus::prelude::*;

#[component]
pub fn BlockVisualizer(block_height: u32) -> Element {
    rsx!({ Chart() })
}
