#![allow(non_snake_case)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use dioxus::prelude::*;
use dioxus_fullstack::Config;
use dioxus_logger::tracing;

use routes::blockvisualizer::BlockVisualizer;
use routes::home::HomePage;

mod routes {
    pub mod blockvisualizer;
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
    #[layout(NavBar)]
        #[route("/")]
        HomepageWrapper {},
        #[route("/block/:block_height")]
        BlockVisualizerWrapper { block_height: u32 },
    #[end_layout]
    #[route("/:route")]
    PageNotFound { route: String },
}

fn App() -> Element {
    rsx! {
        div { class: "bg-cover bg-no-repeat bg-center bg-fixed", style:"background-image: url('/background.jpg')",

            div {class:" container mx-auto min-h-screen min-w-screen",
                br{}
                Router::<Route> {}
            }
            { Footer() }
        }
    }
}

/* Homepage wrapper */
#[component]
fn HomepageWrapper() -> Element {
    rsx!(
        // h1 { "Page found" }
        { HomePage() }
    )
}

#[component]
fn BlockVisualizerWrapper(block_height: u32) -> Element {
    rsx!({
        BlockVisualizer(routes::blockvisualizer::BlockVisualizerProps {
            block_height: block_height,
        })
    })
}
#[component]
fn NavBar() -> Element {
    let mut block_height = use_signal(|| 0);
    let navigator = use_navigator();
    let mut dropdown_menu_toggle = use_signal(|| false);
    let mut dropdown_menu_style = use_signal(|| {
        "visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"
    });

    rsx! {
            nav {class:"bg-opacity-10 bg-white backdrop-filter backdrop-blur-md rounded-full shadow-lg py-2 mx-4", id:"navbar-default",
                div {class:"grid grid-cols-3 sm:grid-cols-1 justify-items-center items-center ps-2 mx-2",

                    div {class:"sm:hidden col-start-1 col-span-1 justify-self-start", h1 {class:"text-slate-200 text-bold", "ErgoVisual"}}

                    button { onclick: move |_| {
                            if dropdown_menu_toggle() {
                                dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");
                                dropdown_menu_toggle.set(false);
                            }
                            else {
                                dropdown_menu_style.set("visibility: visible; opacity: 1; transition: visibility 1s, opacity 0.2s linear");
                                dropdown_menu_toggle.set(true);
                            }
                        },
                        r#type:"button", class:" sm:hidden col-start-3 col-span-1 justify-self-end items-center p-2 w-10 h-10 text-sm text-slate-200 rounded-full hover:bg-slate-100/50 focus:outline-none focus:ring-2 focus:ring-slate-200",
                            svg {class:"h-6 w-6", fill:"none", stroke:"currentColor", "viewBox":"0 0 24 24", xmlns:"http://www.w3.org/2000/svg",
                                path {"stroke-linecap":"round", "stroke-linejoin":"round", "stroke-width":"2", d:"M4 6h16M4 12h16m-7 6h7"}
                            }
                    }

                    div {
                        div {
                            class: "sm:hidden absolute right-0 z-50 m-4 w-56 origin-top-right bg-opacity-10 bg-white backdrop-filter backdrop-blur-md rounded-lg shadow-lg space-x-4 py-2 justify-end items-center text-center content-center ",
                            style: "{dropdown_menu_style}",
                            id: "dropdown_menu",
                            div {class:"grid grid-rows-5 justify-center items-center",
                                div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);},to: Route::HomepageWrapper {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Home"}}

                                div { Link {onclick: move |_| {/*dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);*/},to: Route::HomepageWrapper {}, class:"font-bold text-gray-500 rounded-lg hover:text-slate-100 m-2 ", "Mempool Visualizer"}}

                                div { Link {onclick: move |_| {/*dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);*/},to:"/", class:"font-bold text-gray-500 rounded-lg hover:text-slate-100 m-2 ", "Support"}}

                                div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);},to:"https://explorer.ergoplatform.com/payment-request?address=9fFzKA2WHNYyXZWc4MHPtSv6YqS8jtDsZkSnAQwVaAZrYn9ojEA", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 underline decoration-red-500 m-2 ", "Donate"}}

                                div {
                                    form {
                                        onsubmit: move |_| {
                                            navigator.push(Route::BlockVisualizerWrapper { block_height: block_height() });
                                            dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");
                                            dropdown_menu_toggle.set(false);
                                        },
                                        div {
                                            input {
                                                r#type: "text",
                                                class: "bg-white/30 border py-2 px-2 border-slate-300 placeholder-slate-100 focus:outline-none focus:border-slate-500 focus:ring-slate-300 block w-full rounded-full sm:text-sm focus:ring-1",
                                                placeholder: "Enter block height",
                                                name: "blockheight",
                                                oninput: move |input| {
                                                    block_height.set(input.value().parse::<u32>().unwrap());
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {class:"hidden sm:block sm:grid sm:grid-cols-5 justify-items-center items-center text-center content-center sm:h-fit sm:w-full",
                        div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);},to: Route::HomepageWrapper {}, class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 m-2 ", "Home"}}

                        div { Link {onclick: move |_| {/*dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);*/},to: Route::HomepageWrapper {}, class:"font-bold text-gray-500 rounded-lg hover:text-slate-100 m-2 ", "Mempool Visualizer"}}

                        div { Link {onclick: move |_| {/*dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);*/},to:"/", class:"font-bold text-gray-500 rounded-lg hover:text-slate-100 m-2 ", "Support"}}

                        div { Link {onclick: move |_| {dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear"); dropdown_menu_toggle.set(false);},to:"https://explorer.ergoplatform.com/payment-request?address=9fFzKA2WHNYyXZWc4MHPtSv6YqS8jtDsZkSnAQwVaAZrYn9ojEA", class:"font-bold text-slate-200 rounded-lg hover:text-slate-100 underline decoration-red-500 m-2 ", "Donate"}}

                        div {
                            form {
                                onsubmit: move |_| {
                                    navigator.push(Route::BlockVisualizerWrapper { block_height: block_height() });
                                    dropdown_menu_style.set("visibility: hidden; opacity: 0; transition: visibility 1s, opacity 0.2s linear");
                                    dropdown_menu_toggle.set(false);
                                },
                                div {
                                    input {
                                        r#type: "text",
                                        class: "bg-white/30 border py-2 px-2 border-slate-300 placeholder-slate-100 focus:outline-none focus:border-slate-500 focus:ring-slate-300 block w-full rounded-full sm:text-sm focus:ring-1",
                                        placeholder: "Enter block height",
                                        name: "blockheight",
                                        oninput: move |input| {
                                            block_height.set(input.value().parse::<u32>().unwrap());
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
        }
        if dropdown_menu_toggle() {
            br{}
            br{}
            br{}
            br{}
            br{}
            br{}
            br{}
        }

        Outlet::<Route> {}
    }
}
#[component]
fn Footer() -> Element {
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

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
