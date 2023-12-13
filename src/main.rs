use dioxus::prelude::*;
mod info_screen;
mod instrument;
mod note;
use info_screen::get_info;

#[tokio::main]
async fn main() {
    let mut vdom = VirtualDom::new(app);
     dioxus_desktop::launch(app);
}


fn app(cx: Scope) -> Element {
    let element = get_info(cx);
    cx.bump();
    element
}