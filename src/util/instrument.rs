pub mod instrument {

    use strum_macros::EnumString;


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
}