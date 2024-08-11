#![allow(non_snake_case)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use charming::{
    component::{Axis, Legend, Type},
    element::{AxisType, ItemStyle},
    series::{Line, Pie, PieRoseType, Sankey, SankeyLink},
    Chart, WasmRenderer,
};
use dioxus::prelude::*;
use dioxus_fullstack::Config;
use dioxus_logger::tracing;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    let debug_flag = true;
    let serve_on_addr: SocketAddr;
    if debug_flag {
        serve_on_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8070);
    } else {
        serve_on_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8070);
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

#[component]
fn Home() -> Element {
    spawn(async move {
        // let chart = Chart::new()
        //     .x_axis(
        //         Axis::new()
        //             .type_(AxisType::Category)
        //             .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        //     )
        //     .y_axis(Axis::new().type_(AxisType::Value))
        //     .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        let chart = Chart::new().legend(Legend::new().top("bottom")).series(
            Sankey::new()
                .name("Block")
                .data(vec!["1", "2", "3", "a1", "b2", "c3", "block"])
                .links(vec![
                    SankeyLink {
                        source: "1".to_string(),
                        target: "block".to_string(),
                        value: 5.0,
                    },
                    SankeyLink {
                        source: "2".to_string(),
                        target: "block".to_string(),
                        value: 3.0,
                    },
                    SankeyLink {
                        source: "3".to_string(),
                        target: "block".to_string(),
                        value: 2.0,
                    },
                    SankeyLink {
                        source: "block".to_string(),
                        target: "a1".to_string(),
                        value: 2.0,
                    },
                    SankeyLink {
                        source: "block".to_string(),
                        target: "b2".to_string(),
                        value: 6.0,
                    },
                    SankeyLink {
                        source: "block".to_string(),
                        target: "c3".to_string(),
                        value: 2.0,
                    },
                ]),
        );
        let renderer = WasmRenderer::new(1000, 800);

        renderer.render("chart", &chart).unwrap();
    });

    rsx! {
        div {
            style: "width: 100%; text-align: center;",
            div {
                id: "chart",
                style: "display: inline-block;",
            }
    }}
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
