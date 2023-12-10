use dioxus_core::*;
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
   // let instrument = instrument_type(cx);
   // string_count(cx);
    cx.render(rsx! {
        div {
            instrument_type(cx),
            string_count(cx),
        }
    })

}
#[derive(Debug)]
enum Instrument {
    GUITAR,
    BASS,
    UKELELE,
    MANDOLIN,
    BANJO,
    KEYBOARD,
}

fn instrument_type(cx: Scope) -> Element {
    let instrument = use_state(cx, || Instrument::BASS);
cx.render(
    rsx! {
        div {
            h1 { format!("Instrument: {:?}", *instrument) }
            button {
                onclick: move |_| instrument.set(Instrument::GUITAR),
                "Guitar"
            }
            button {
                onclick: move |_| instrument.set(Instrument::BASS),
                "Bass"
            }
            // Add buttons for other instruments
        }
    })
}

fn string_count(cx: Scope) -> Element {
    let strings = use_state(cx, || 0);

  cx.render(rsx! {
        div {
            h1 { format!("String Counter: {}", *strings) }
            button {
                onclick: move |_| strings.set(6),
                "6 String"
            }
            button {
                onclick: move |_| strings.set(7),
                "7 String"
            }
            button {
                onclick: move |_| strings.set(8),
                "8 String"
            }
        }
    })
}
