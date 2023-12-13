pub mod instrument {
    use std::iter::Iterator;
    use dioxus::prelude::*;
    use dioxus_core::DynamicNode;
    use dioxus_html::div;
    use strum_macros::EnumString;
    use crate::note::note::{Note, NoteName, TuningType};
    use std::option::Option::Some;
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
        pub notes: Vec<Vec<Note>>,
        pub number_of_strings: u8,
        pub number_of_frets: u8,
    }

    impl Iterator for FretBoard {
        type Item = Note;
        fn next(&mut self) -> Option<Self::Item> {
            let mut current_index = 0 as usize;
            if current_index <= self.number_of_strings as usize {
            let mut current_jdex = 0 as usize;
            if current_jdex <=self.number_of_frets as usize {
             current_jdex+=1;
                let note = self.notes[current_index][current_jdex].clone();
               return Some(note);
              }
             current_index+=1;
            }
             return None;
            }


    }

    impl FretBoard {
        fn new(number_of_strings: u8, number_of_frets: u8) -> Self {
            FretBoard {
                notes: Self::generate(number_of_strings,number_of_frets),
                number_of_strings,
                number_of_frets,
            }
        
        }
        fn generate(number_of_strings : u8 ,number_of_frets : u8) -> Vec::<Vec::<Note>> {
        let mut vector = Vec::new();
        for i in 1..=(number_of_strings+1){
        let mut fret_vector = Vec::new();
        for j in 1..=(number_of_frets+1){
        fret_vector = Note::note_setter(&mut fret_vector, TuningType::Drop, NoteName::D, i, j);
        }
        vector.push(fret_vector);
        }
        vector.to_vec()
        }
        pub fn into_vnode<'a>(&'a self, cx: &'a ScopeState) -> Element{
            // Render the first note and create a DynamicNode from it
          
            render!(
                for musicalString in self.notes.iter() {
                for musicalNote in musicalString.iter(){

                  musicalNote.into_vnode(cx)

                }
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
