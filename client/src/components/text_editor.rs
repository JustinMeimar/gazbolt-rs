use crate::state::{AppAction, AppState};
use yew::prelude::*;
use crate::components::run_button::RunButton;
use crate::components::program_dropdown::ProgramDropdown;
use crate::components::compiler_dropdown::CompilerDropdown;

#[derive(Properties, PartialEq)]
pub struct TextEditorProps {
    #[prop_or_default]
    pub value: String,
}

#[function_component]
pub fn TextEditor(_props: &TextEditorProps) -> Html {
    
    let app_state = use_context::<UseReducerHandle<AppState>>()
        .expect("No State found");

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
        <CompilerDropdown/>
        <ProgramDropdown/>
        <RunButton/>
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

