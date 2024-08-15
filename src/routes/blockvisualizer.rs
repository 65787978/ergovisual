use crate::{data::data::get_server_data, utils::chart::Chart};
use dioxus::prelude::*;
use gloo::timers::future::TimeoutFuture;

#[component]
pub fn BlockVisualizer(block_height: u32) -> Element {
    let mut server_data = use_resource(move || async move { get_server_data().await });

    use_future(move || async move {
        loop {
            TimeoutFuture::new(5000).await;
            server_data.restart()
        }
    });

    match &*server_data.read_unchecked() {
        Some(Ok(data)) => rsx!("{data:?}"),
        Some(Err(err)) => rsx!("{err:?}"),
        None => rsx!(),
    }
}
