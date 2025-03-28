use core::{ApiExecResponse, ApiExecRequest};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use crate::state::{AppState, AppAction};
use crate::components::editor_controls::EditorControls;

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

  let on_run = {
    let app_state = app_state.clone(); 
    Callback::from(move |_e: MouseEvent| {
      let app_state = app_state.clone();
      let code = app_state.code.clone(); 
      spawn_local(async move {        
        let request_body = ApiExecRequest { code };
         
        match Request::post("http://127.0.0.1:3000/api/run/gcc")
          .header("Content-Type", "application/json")
          .json(&request_body)
          .expect("Faield to serialize request body")
          .send()
          .await
        {
          Ok(response) => match response.status() {
            200 => {
              console::log_1(&"POST received 200".into());
              match response.json::<ApiExecResponse>().await {
                Ok(exec_response) => {
                  app_state.dispatch(AppAction::UpdateStdout(exec_response.stdout));
                  app_state.dispatch(AppAction::UpdateStderr(exec_response.stderr));
                }
                Err(e) => {
                  console::log_1(&format!("Failed to deserialize: {}", e).into());
                }
              } 
            }
            404 => {
              console::log_1(&"GET received 404".into());
            }
            _ => {
              console::log_1(&format!("GET received: {}", response.status()).into());
            }
          },
          Err(e) => {
              eprintln!("Error: {:?}", e);
          }
        }
      });
      console::log_1(&"me".into());
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
      <button onclick={on_run}>
      {"Run"}
      </button>
    </div>
  }
}
