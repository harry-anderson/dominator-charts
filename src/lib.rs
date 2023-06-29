use dominator::{class, clone, events, html, svg, with_node, Dom};
use futures_signals::signal::{not, Mutable};
use gloo_timers::future::TimeoutFuture;
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use util::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

mod util;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_: String,
    site_admin: bool,
    name: Option<String>,
    company: Option<String>,
    blog: String,
    location: Option<String>,
    email: Option<String>,
    hireable: Option<bool>,
    bio: Option<String>,
    public_repos: u32,
    public_gists: u32,
    followers: u32,
    following: u32,
    created_at: String,
    updated_at: String,
}

impl User {
    async fn fetch(user: &str) -> Result<Self, JsValue> {
        let user = fetch_github(&format!("https://api.github.com/users/{}", user)).await?;
        Ok(serde_json::from_str::<Self>(&user).unwrap())
    }
}

struct App {
    user: Mutable<Option<User>>,
    input: Mutable<String>,
    loader: AsyncLoader,
}

impl App {
    fn new(name: &str, user: Option<User>) -> Arc<Self> {
        Arc::new(Self {
            user: Mutable::new(user),
            input: Mutable::new(name.to_string()),
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
                              .attr("points", "
                                    0, 250
                                    20, 60
                                    140, 80
                                    260, 160
                                    480, 20
                                    500, 0
                                ")
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

    let user = User::fetch("Pauan").await.ok();

    let app = App::new("Pauan", user);

    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
