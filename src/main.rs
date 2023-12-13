mod instrument;
use dioxus_core::*;
use dioxus::prelude::*;
use crate::instrument::instrument::instrument::{Bridge, BridgedInstrumentType, FretBoard, Instrument, InstrumentType};
fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    get_info(cx)
}
fn get_info(cx: Scope) -> Element {
let (instrument_type_element, instrument_type) = get_instrument_type(cx);
    let (bridged_instrument_type_element, bridged_instrument_type) =
        get_bridged_instrument_type(cx, instrument_type);
    let (fretcount_element, fret_count) = get_fretcount(cx, bridged_instrument_type);
    let (instrument_element, instrument) = generate_instrument(cx, bridged_instrument_type, fret_count);
    cx.render(rsx! { 
        div { 
            instrument_type_element, 
            bridged_instrument_type_element, 
            fretcount_element 
        }
    })
    }
 pub fn generate_instrument(cx : Scope, bridged_instrument_type : BridgedInstrumentType, number_of_frets: u8) -> (Element, Instrument){
        let instrument = use_state(cx, ||Instrument::new(bridged_instrument_type, number_of_frets));

        let element = cx.render(rsx!{
            button {
                onclick : |_| todo!() //generate(cx, &mut instrument)
                , "GENERATE"
            }
        });
        (element, (**instrument).clone())
    }
    /*fn generate(cx: Scope, instrument : &mut Instrument) -> Element {
        cx.render(rsx!{a {"yada"}})
    }*/
fn get_fretcount(cx: Scope, instrument: BridgedInstrumentType) -> (Element, u8){
    let fret_count = use_state(cx, || 22);
    let element = cx.render(rsx!{
        h1 {format!("Frets: {}", fret_count)}
        button {
            onclick : move |_| fret_count.set(22), "22 Frets"
        }
        button {

            onclick : move |_| fret_count.set(24), "24 Frets"
        }
    });
    (element, **fret_count)
}

fn get_instrument_type(cx: Scope) -> (Element, InstrumentType) {
    let instrument_type = use_state(cx, || InstrumentType::GUITAR);
    let element = cx.render(rsx! {
            div {
                h1 { format!("Instrument: {:?}", instrument_type) }
                button {
                    onclick: move |_| instrument_type.set(InstrumentType::GUITAR),
                    "Guitar"
                }
                button {
                    onclick: move |_| instrument_type.set(InstrumentType::BASS),
                    "Bass"
                }
                 button {
                     onclick: move |_| instrument_type.set(InstrumentType::MANDOLIN),
                    "Mandolin"
                 }
                  button {
                      onclick: move |_| instrument_type.set(InstrumentType::UKELELE),
                      "Ukelele"
                  }
                   button {
                       onclick: move |_| instrument_type.set(InstrumentType::BANJO),
                       "Banjo"
                   }
                   button {
                        onclick: move |_| instrument_type.set(InstrumentType::KEYBOARD),
                        "keyboard"
                   }
            }
        });

        (element, **instrument_type)
}


fn get_bridged_instrument_type(
    cx: Scope,
    instrument_type: InstrumentType,
) -> (Element, BridgedInstrumentType) {
    if instrument_type == InstrumentType::GUITAR {
        let bridged_instrument_type = use_state(cx, || {
            BridgedInstrumentType::new(Bridge::new(6), InstrumentType::GUITAR)
        });

        let element = cx.render(rsx! {
            div {
                h1 { format!("String Counter: {}", bridged_instrument_type.bridge.string_count) }
                button {
                    onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(6), InstrumentType::GUITAR)),
                    "6 String"
                }
                button {
                    onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(7), InstrumentType::GUITAR)),
                    "7 String"
                }
                button {
                    onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(8), InstrumentType::GUITAR)),
                    "8 String"
                }
                button {
                    onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(12), InstrumentType::GUITAR)),
                    "12 String"
                }
            }
        });
        (element, **bridged_instrument_type)
    } else if instrument_type == InstrumentType::BASS {
        let bridged_instrument_type = use_state(cx, || {
            BridgedInstrumentType::new(Bridge::new(4), InstrumentType::BASS)
        });
       let element = cx.render(rsx! {
                           div {
                               h1 { format!("String Counter: {}", bridged_instrument_type.bridge.string_count) }
                               button {
                                   onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(4), InstrumentType::BASS)),
                                   "4 String"
                               }
                               button {
                                   onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(5), InstrumentType::BASS)),
                                   "5 String"
                               }
                               button {
                                   onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(6), InstrumentType::BASS)),
                                   "6 String"
                               }
                                 button {
                                    onclick: move |_| bridged_instrument_type.set(BridgedInstrumentType::new(Bridge::new(7), InstrumentType::BASS)),
                                    "7 String"
                               }
                           }
                       });

        (element, **bridged_instrument_type)
    } else {
        (
            cx.render(rsx! { div { h1 { "Error" } } }),
            BridgedInstrumentType::new(Bridge::new(6), InstrumentType::GUITAR),
        )
    }
}

