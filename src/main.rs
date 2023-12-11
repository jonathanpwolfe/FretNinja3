mod util;
use dioxus_core::*;
use dioxus::prelude::*;
use crate::util::instrument::instrument::InstrumentType;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let (instrument_element, instrument) = get_instrument_type(cx);
    let (bridged_instrument_type_element, bridged_instrument_type) =
        get_bridged_instrument_type(cx, instrument);
    //let mut fretboard = get_fretboard(cx, bridged_instrument_type);

    cx.render(rsx! { div { instrument_element, bridged_instrument_type_element } })

    //fretboard.build();
    //fretboard.render(cx)
}

fn get_fretboard(cx: Scope, instrument: BridgedInstrumentType) -> (Element, FretBoard) {
    // Implement the logic for creating a FretBoard based on the BridgedInstrumentType
    unimplemented!()
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
#[derive(Debug, Copy, Clone)]
struct Bridge {
    string_count: u8,
}

impl Bridge {
    fn new(string_count: u8) -> Self {
        Bridge { string_count }
    }
}

#[derive(Debug, Copy, Clone)]
struct BridgedInstrumentType {
    instrument_type: InstrumentType,
    bridge: Bridge,
}

impl BridgedInstrumentType {
    fn new(bridge: Bridge, instrument_type: InstrumentType) -> Self {
        BridgedInstrumentType {
            instrument_type,
            bridge,
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct FretBoard {}

impl FretBoard {
    fn build(&mut self) {
        // Implement FretBoard building logic
    }

   // fn render(&self, cx: Scope) -> Element {
    //    cx.render(rsx! { div { format!("{:?}",self) } })
 //   }
}
#[derive(Debug, Copy, Clone)]
 struct Instrument{
        instrument_type: InstrumentType,
        bridge: Bridge,
        number_of_frets : u8,

    }
 impl Instrument{

    fn new(bridged_instrument_type: BridgedInstrumentType, number_of_frets: u8) -> Self{
        Instrument{
            instrument_type : bridged_instrument_type.instrument_type,
            bridge : bridged_instrument_type.bridge,
            number_of_frets : number_of_frets
        }
    }
   }

