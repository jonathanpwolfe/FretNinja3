pub mod note {
    use dioxus::prelude::*;
    use dioxus::prelude::{IntoDynNode, Scope, render, VNode};
    use shipyard::Component;
    use dioxus_core::DynamicNode;

    #[derive(Debug, Clone, Component)]
    pub struct Note {
        note_name: NoteName,
    }
    impl Note {
   pub fn render<'a>(&self, cx: Scope<'a>) -> DynamicNode<'a> {
        ScopeState::make_node(cx, rsx! {format!("{{<span>{:?}</span>}}", self.note_name)})
    }
}


    use strum_macros::EnumString;

    #[derive(EnumString, Clone, Debug)]
    pub enum NoteName {
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
