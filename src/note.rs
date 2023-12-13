pub mod note {

    use strum_macros::EnumString;
    #[derive(EnumString)]
    pub enum Note {
        #[strum(serialize = "A")]
        A,
        #[strum(serialize = "A#")]
        ASharp,
        #[strum(serialize = "B")]
        B,
        #[strum(serialize = "C")]
        C,
        #[strum(serialize = "C#")]
        CSharp,
        #[strum(serialize = "D")]
        D,
        #[strum(serialize = "D#")]
        DSharp,
        #[strum(serialize = "E")]
        E,
        #[strum(serialize = "F")]
        F,
        #[strum(serialize = "F#")]
        FSharp,
        #[strum(serialize = "G")]
        G,
        #[strum(serialize = "G#")]
        GSharp,
    }
}