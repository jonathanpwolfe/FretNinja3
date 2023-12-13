mod info_screen;
mod fretboard_screen;
mod instrument;
mod note;
mod screen;

use dioxus::prelude::*;
use dioxus_elements::{link, button};
use info_screen::get_info;
use screen::Screen;
use shipyard::Component;
use instrument::instrument::{InstrumentType, Bridge, BridgedInstrumentType, Instrument};

#[tokio::main]
async fn main() {
    let mut stuff = Stuff::new();
    dioxus_desktop::launch(app);
}

#[derive(Debug, PartialEq)]
struct AppState {
    current_screen: Screen,
}

struct Stuff {
    instrument: Instrument,
    state: AppState,
}

impl Stuff {
    fn new() -> Self {
        Stuff {
            instrument: Instrument::new(
                BridgedInstrumentType {
                    instrument_type: InstrumentType::GUITAR,
                    bridge: Bridge { string_count: 6 },
                },
                22,
            ),
            state: AppState {
                current_screen: Screen::Info,
            },
        }
    }
}
    fn app(cx: Scope) -> Element {
    let mut stuff : Stuff = Stuff::new();
        let (screen, instrument, fretboard_info) = get_info(cx.clone());
        stuff.state.current_screen = screen;

        match stuff.state.current_screen {
            Screen::Info => {
                // Use rsx! macro to render fretboard_info

                // Use rsx! macro to render instrumen

                // Combine both elements into a single Element
                cx.render(rsx! {
                    div {
                    fretboard_info
                        button {
                            onclick: move |_| stuff.state.current_screen = Screen::Fretboard,
                            "Switch to Fretboard"
                        }
                    }
                })
            }
            Screen::Fretboard => {
                // Logic for rendering the fretboard screen
                // You might call a different function or component
                // depending on the desired behavior for the fretboard screen.
                // Example:
                // fretboard_screen::render_fretboard(self.link.clone())
                // ...
                let instrument_element = instrument.fretboard.clone();
                // Placeholder: Just returning a simple div for demonstration purposes
                cx.render(rsx! {
                    div {
                        button {
                            onclick: move |_| stuff.state.current_screen = Screen::Info,
                            "Switch to Info"
                        }
                    }
                })
            }
        }
  }
