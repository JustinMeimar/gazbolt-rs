use crate::components::editor_controls::EditorControls;
use crate::state::{AppAction, AppState};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextEditorProps {
    #[prop_or_default]
    pub value: String,
}

#[function_component]
pub fn TextEditor(_props: &TextEditorProps) -> Html {
    let app_state = use_context::<UseReducerHandle<AppState>>().expect("No State found");

    let on_code_change = {
        // copy to move into closure
        let app_state = app_state.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>();
            if let Some(input) = input {
                app_state.dispatch(AppAction::UpdateCode(input.value().into()));
            }
        })
    };

    html! {
      <div>
        { "This is the code editor" }
        <EditorControls/>
        <textarea
          width={"300"}
          height={"500"}
          rows={"24"}
          value={app_state.code.clone()}
          onchange={on_code_change}
        />
        
      </div>
    }
}
