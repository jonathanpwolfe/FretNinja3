use dioxus::prelude::*;
mod info_screen;
mod instrument;
use info_screen::get_info;

#[tokio::main]
async fn main() {
    dioxus_desktop::launch(app);
}

struct Note {}



fn app(cx: Scope) -> Element {
    let (element, _) = get_info(cx);
    element
}


