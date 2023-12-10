mod util;
use dioxus_core::*;
use dioxus::prelude::*;
use crate::util::instrument::instrument::Instrument;
fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
   let (instrument_element, instrument) = instrument_type(cx);
   let strings_element = string_count(cx);
   cx.render(rsx!{div{
   instrument_element,
   strings_element,
   }
   })


}


fn instrument_type(cx: Scope) -> (Element, Instrument) {
    let instrument = use_state(cx, || Instrument::GUITAR);
    let element = cx.render(rsx! {
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
    });

    (element, **instrument)
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

