pub mod note {
    use dioxus::prelude::*;
    use dioxus::prelude::{IntoDynNode, Scope, render, VNode};
    use shipyard::Component;
    use dioxus_core::DynamicNode;
    use dioxus_html::button;
    use dioxus::events::MouseData;
    #[derive(Debug, Clone, PartialEq, Component)]
    pub struct Note {
        note_name: NoteName,
        }
  impl Note {
pub fn into_vnode<'a>(&'a self, cx: &'a ScopeState) -> Element<'a> {
    render!(rsx!{button{
     onclick: move |_| self.highlight(cx),
     {format!("{:?}", self.note_name).to_string()}
}
   } )
}



fn highlight(self: &Self, cx : &ScopeState){
}



   pub fn render<'a>(&self, cx: Scope<'a>) -> Element<'a> {
     cx.render( rsx!{format!("{{<span>{:?}</span>}}", self.note_name)})

    }
}





    use strum_macros::EnumString;






    #[derive(EnumString,PartialEq, Clone, Debug)]
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
