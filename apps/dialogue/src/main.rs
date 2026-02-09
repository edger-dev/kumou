use dioxus::prelude::*;

use views::{DialogueDetail, Navbar, TopicDialogues, TopicList};

mod components;
mod server_fns;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        TopicList {},
        #[route("/topic/:topic_id")]
        TopicDialogues { topic_id: u32 },
        #[route("/dialogue/:dialogue_id")]
        DialogueDetail { dialogue_id: u32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
