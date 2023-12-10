mod util;
use dioxus_core::*;
use dioxus::prelude::*;
use crate::util::instrument::instrument::Instrument;
fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
   let (instrument_element, instrument) = instrument_type(cx);
   let strings_element = string_count(cx, instrument);
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
             button {
                 onclick: move |_| instrument.set(Instrument::MANDOLIN),
                "Mandolin"
             }
              button {
                  onclick: move |_| instrument.set(Instrument::UKELELE),
                  "Ukelele"
              }
               button {
                   onclick: move |_| instrument.set(Instrument::BANJO),
                   "Banjo"
               }
               button {
                    onclick: move |_| instrument.set(Instrument::KEYBOARD),
                    "keyboard"
               }
        }
    });

    (element, **instrument)
}


fn string_count(cx: Scope, instrument : Instrument) -> Element {

   if instrument == Instrument::GUITAR{
   let strings = use_state(cx, || 6);
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
                  button {
                     onclick: move |_| strings.set(12),
                     "12 String"
                }
            }
        })
        }
       else if instrument == Instrument::BASS{
       let strings = use_state(cx, || 4);
                cx.render(rsx! {
                    div {
                        h1 { format!("String Counter: {}", *strings) }
                        button {
                            onclick: move |_| strings.set(4),
                            "4 String"
                        }
                        button {
                            onclick: move |_| strings.set(5),
                            "5 String"
                        }
                        button {
                            onclick: move |_| strings.set(6),
                            "6 String"
                        }
                          button {
                             onclick: move |_| strings.set(7),
                             "7 String"
                        }
                    }
                })
    }
    else
    {cx.render(rsx!{
    div {
    h1 {format!("error")}
    }
    })}
}
