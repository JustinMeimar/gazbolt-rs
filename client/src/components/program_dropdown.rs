use core::{ApiCompilerListView, ApiExecRequest, ApiExecResponse};
use crate::state::{AppState, AppAction};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[function_component]
pub fn ProgramDropdown() -> Html {
    
    let app_state = use_context::<UseReducerHandle<AppState>>().expect("No State found");
    let program_options = use_state(|| vec!["loop.in".to_string(), "vec.in".to_string()]);
     
    html! {
      <div class="program-dropdown">
          <select>
          { program_options.iter().map(|(program)| {
            html! { <option value={program.clone()}>{program}</option> }
          }).collect::<Html>() }
        </select> 
      </div>
    }
}

