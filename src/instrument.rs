pub mod instrument {
    use std::iter::Iterator;
    use dioxus::prelude::*;
    use dioxus_core::DynamicNode;
    use dioxus_html::div;
    use strum_macros::EnumString;
    use crate::note::note::Note;

    #[derive(Debug, Copy, Clone, PartialEq, EnumString)]
    pub enum InstrumentType {
        #[strum(serialize = "Guitar")]
        GUITAR,
        #[strum(serialize = "Bass")]
        BASS,
        #[strum(serialize = "Ukelele")]
        UKELELE,
        #[strum(serialize = "Mandolin")]
        MANDOLIN,
        #[strum(serialize = "Banjo")]
        BANJO,
        #[strum(serialize = "Keyboard")]
        KEYBOARD,
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Bridge {
        pub string_count: u8,
    }

    impl Bridge {
        pub fn new(string_count: u8) -> Self {
            Bridge { string_count }
        }
    }

    #[derive(Debug, Copy, PartialEq, Clone)]
    pub struct BridgedInstrumentType {
        pub instrument_type: InstrumentType,
        pub bridge: Bridge,
    }

    impl BridgedInstrumentType {
        pub fn new(bridge: Bridge, instrument_type: InstrumentType) -> Self {
            BridgedInstrumentType { instrument_type, bridge }
        }
    }

    // ... (imports remain unchanged)

    #[derive(Debug, PartialEq, Clone)]
    pub struct FretBoard {
        pub notes: Vec<Note>,
        pub number_of_strings: u8,
        pub number_of_frets: u8,
    }

    impl Iterator for FretBoard {
        type Item = Note;

        fn next(&mut self) -> Option<Self::Item> {
            let current_index = 0;
            if current_index < self.notes.len() {
                let note = self.notes[current_index].clone();
                Some(note)
            } else {
                None
            }
        }
    }

    impl FretBoard {
        fn new(number_of_strings: u8, number_of_frets: u8) -> Self {
            FretBoard {
                notes: Vec::<Note>::new(),
                number_of_strings,
                number_of_frets,
            }
        
        }
        
        pub fn into_vnode<'a>(&'a self, cx: &'a ScopeState) -> Element{
            // Render the first note and create a DynamicNode from it
          
            render!(  
                for note in  self.notes.iter() {
                  note.into_vnode(cx)
                }
                )
        }
    }






























    #[derive(Debug, Clone, PartialEq)]
    pub struct Instrument {
        pub instrument_type: InstrumentType,
        pub bridge: Bridge,
        pub number_of_strings: u8,
        pub number_of_frets: u8,
        pub fretboard: FretBoard,
    }

    impl Instrument {
        pub fn new(bridged_instrument_type: BridgedInstrumentType, number_of_frets: u8) -> Self {
            let fretboard: FretBoard =
                FretBoard::new(bridged_instrument_type.bridge.string_count, number_of_frets);
            Instrument {
                instrument_type: bridged_instrument_type.instrument_type,
                bridge: bridged_instrument_type.bridge,
                number_of_strings: bridged_instrument_type.bridge.string_count,
                number_of_frets,
                fretboard,
            }
        }
    }
}
