use charming::{
    component::Legend,
    series::{Sankey, SankeyLink},
    Chart, WasmRenderer,
};
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn Chart() -> Element {
    spawn(async move {
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
        tracing::info!("rendered chart");
    });

    rsx! (
        div {class:"bg-opacity-10 bg-white backdrop-filter backdrop-blur-2xl rounded-lg shadow-lg p-2 mt-6",
            style: "width: 100%; text-align: center;",
            div {
                id: "chart",
                style: "display: inline-block;",
            }
        }
    )
}
