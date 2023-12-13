

   
   pub mod instrument{
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
#[derive(Debug, Copy, Clone)]
pub struct Bridge {
   pub string_count: u8,
}

impl Bridge {
   pub fn new(string_count: u8) -> Self {
        Bridge { string_count }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BridgedInstrumentType {
    pub instrument_type: InstrumentType,
   pub bridge: Bridge,
}

impl BridgedInstrumentType {
   pub fn new(bridge: Bridge, instrument_type: InstrumentType) -> Self {
        BridgedInstrumentType {
            instrument_type,
            bridge,
        }
    }
}
#[derive(Debug, Clone)]
pub struct FretBoard {
    pub notes : Vec<Note>,
    pub number_of_strings : u8,
    pub number_of_frets : u8,
}
    

impl FretBoard {
    fn new(number_of_strings :u8, number_of_frets : u8) -> Self {
        FretBoard{
            notes : Vec::<Note>::new(),
            number_of_strings : number_of_strings,
            number_of_frets : number_of_frets,
        }

    }

   // fn render(&self, cx: Scope) -> Element {
    //    cx.render(rsx! { div { format!("{:?}",self) } })
 //   }
}
#[derive(Debug, Clone)]
 pub struct Instrument{
       pub instrument_type: InstrumentType,
       pub bridge: Bridge,
       pub number_of_strings: u8,
       pub number_of_frets : u8,
       pub fretboard : FretBoard,

    }
 impl Instrument{

    pub fn new(bridged_instrument_type: BridgedInstrumentType, number_of_frets: u8) -> Self{
        let fretboard : FretBoard = FretBoard::new(bridged_instrument_type.bridge.string_count, number_of_frets);
        Instrument{
            instrument_type : bridged_instrument_type.instrument_type,
            bridge : bridged_instrument_type.bridge,
            number_of_strings : bridged_instrument_type.bridge.string_count,
            number_of_frets : number_of_frets,
            fretboard : fretboard,

        }
    }
    pub fn generate(self: &Self) {

    }
   
   }
  
    }