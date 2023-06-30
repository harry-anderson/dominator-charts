use dominator::{class, clone, events, html, svg, with_node, Dom};
use futures_signals::signal::{not, Mutable};
use gloo_timers::future::TimeoutFuture;
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use util::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

mod util;

struct App {
    input: Mutable<String>,
    data: Mutable<Value>,
    loader: AsyncLoader,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    points: Vec<[i64; 2]>,
}

impl App {
    fn new(name: &str, data: serde_json::Value) -> Arc<Self> {
        Arc::new(Self {
            input: Mutable::new(name.to_string()),
            data: Mutable::new(data),
            loader: AsyncLoader::new(),
        })
    }

    fn render(app: Arc<Self>) -> Dom {
        static APP: Lazy<String> = Lazy::new(|| {
            class! {
                .style("white-space", "pre")
            }
        });

        static SVG: Lazy<String> = Lazy::new(|| {
            class! {
                .style("background-color", "#ccc")
            }
        });

        html!("div", {
            .class(&*APP)
            .children(&mut [
                  svg!("svg", {
                    .class(&*SVG)
                    .attr("viewBox", "0 0 500 250")
                    .children(&mut [
                          svg!("polyline", {
                              .attr("fill", "none")
                              .attr("stroke", "blue")
                              .attr("stroke-width", "4")
                              .attr_signal("points", app.data.signal_ref(|data| {
                                  match data.pointer("/points") {
                                    Some(Value::Array(arr)) => {
                                        arr.iter().map(|v| {
                                            match v {
                                                Value::Array(arr) => format!("{}, {}",arr[0], arr[1]),
                                                _ => String::from("")
                                            }
                                        }).collect::<Vec<String>>().join(" ")
                                    },
                                    Some(_) => String::from(""),
                                    None => String::from(""),
                                }
                              }))
                          })
                    ])
                  })
            ])
        })
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let data = json!({
        "points": [
            [0, 250],
            [100, 100],
            [200, 150],
            [300, 0],
        ]
    });

    let app = App::new("Pauan", data);

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
