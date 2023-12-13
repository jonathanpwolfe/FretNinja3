
   
    use dioxus::prelude::*;
    use tokio::task::spawn_local;

use crate::instrument:: instrument::{Bridge, BridgedInstrumentType, Instrument, InstrumentType};




    
trait DetachedScope {
    fn new() -> Self;
}

impl DetachedScope for Scope<'_> {
    fn new() -> Self {
        Scope::new()
    }
}
pub fn get_info(cx: Scope) -> (Element, Instrument) {
    let (instrument_type_element, instrument_type) = get_instrument_type(cx);
    let (bridged_instrument_type_element, bridged_instrument_type) =
        get_bridged_instrument_type(cx, instrument_type);
    let (fretcount_element, fret_count) = get_fretcount(cx, bridged_instrument_type);
    let instrument = use_state(cx, || Instrument::new(bridged_instrument_type, fret_count));

    let element = cx.render(rsx! {
        div {
            instrument_type_element,
            bridged_instrument_type_element,
            fretcount_element,
        }
    div {
            button {
                // Assuming you're using the `click` event
                onclick: move |event: MouseEvent| {
                    // Use `async move` to capture variables inside the async block
                    let instrument_clone = (**instrument).clone();
                    // Create a new Scope
                    let detached_scope = Scope::new();
                    // Use tokio::task::spawn_local with the Scope
                    spawn_local(async move {
                        // Use `await` here to wait for the result of the async function
                        generate_fretboard(detached_scope, instrument_clone).await;
                    });
                },
                "Generate"
            }
        }
    });

    (element, (**instrument).clone())
}

async fn generate_fretboard(cx: Scope<'_>, instrument: Instrument) -> (Element, Instrument) {
    // Create a vector of elements using a regular Rust for loop
    let fretboard_elements: Vec<Element> = (0..instrument.number_of_strings)
        .flat_map(|i| (0..instrument.number_of_frets).map(move |j| (i, j)))
        .map(|(i, j)| cx.render(rsx! { p { format!("String {}, Fret {}", i, j) } }))
        .collect();

    // Render the elements within the rsx! macro
    let fretboard = cx.render(rsx! {
        div {
            // You can include other JSX-like elements or components here
            // ...

            // Include the fretboard elements
            { for element in fretboard_elements { 
                element; } }
        }
    });

    // You can update the state or perform other actions based on your logic

    (fretboard, instrument)
}

fn get_fretcount(cx: Scope, instrument: BridgedInstrumentType) -> (Element, u8) {
    let fret_count = use_state(cx, || 22);
    let element = cx.render(rsx! {
        h1 { format!("Frets: {}", fret_count) }
        button {
            onclick: move |_| fret_count.set(22),
            "22 Frets"
        }
        button {
            onclick: move |_| fret_count.set(24),
            "24 Frets"
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
