#![allow(non_snake_case)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dioxus::prelude::*;
use dioxus_fullstack::Config;
use dioxus_logger::tracing;

use routes::home::HomePage;

mod routes {
    pub mod home;
}
mod utils {
    pub mod chart;
}

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
        #[end_layout]
    #[route("/:route")]
    PageNotFound { route: String },
}

/* Homepage wrapper */
#[component]
fn Home() -> Element {
    rsx!(
        h1 { "Page found" }
        { HomePage() }
    )
}

#[component]
fn HeaderFooter() -> Element {
    rsx!()
}

#[component]
fn PageNotFound(route: String) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route}" }
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    let debug_flag = true;

    let mut serve_on_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8070);

    if debug_flag {
        serve_on_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8070);
    }

    // launch_fullstack(app);
    LaunchBuilder::new()
        .with_cfg(server_only! {Config::new().addr(serve_on_addr)})
        .launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
