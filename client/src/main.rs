pub mod compiler;
pub mod state;
use core::{ApiExecResponse, ApiExecRequest, ApiCompilerListView};
use compiler::Compiler;
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use state::{AppAction, AppState};
use std::fmt;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextBoxProps {
  name: AttrValue,
  placeholder: AttrValue,
  #[prop_or_default]
  readonly: bool,
  value: AttrValue,
}

#[function_component]
fn TextBox(props: &TextBoxProps) -> Html {
  let app_state = use_context::<UseReducerHandle<AppState>>()
    .expect("No State found");
  html! {
    <textarea
      name={props.name.clone()}
      placeholder={props.placeholder.clone()}
      readonly={props.readonly}
      value={props.value.clone()}
    />
  }
}

#[derive(Properties, PartialEq)]
pub struct SelectorProps {
  pub class: AttrValue,
  pub options: Vec<(String, String)>,
}

#[function_component]
fn Selector(props: &SelectorProps) -> Html {
  html! {
    <select class={props.class.clone()}>
      {
        props.options.iter().map(|(value, text)| {
          html! {
            <option value={value.clone()}>{text.clone()}</option>
          }
        }).collect::<Html>()
      }
    </select>
  }
}

#[function_component]
fn EditorControls() -> Html {
  let compiler_options = use_state(|| Vec::<(String, String)>::new());

  let options = compiler_options.clone();
  use_effect_with((),
    move |_| {
      let options = options.clone();

      spawn_local(async move {
        match Request::get("http://127.0.0.1:3000/api/compilers")
          .send()
          .await
        {
          Ok(response) => match response.status() {
            200 => {
              console::log_1(&"GET received 200".into());
              match response.json::<ApiCompilerListView>().await {
                Ok(api_response) => {
                  let formatted_options: Vec<(String, String)> = api_response
                      .configs
                      .into_iter()
                      .map(|c| (c.name.clone(), format!("{} {}", c.name, c.version)))
                      .collect();
                  options.set(formatted_options);
                },
                Err(e) => {
                  console::error_1(&format!("Error parsing response: {:?}", e).into());
                }
              }
            }
            status => {
              console::error_1(&format!("Unexpected status: {}", status).into());
            }
          },
          Err(e) => {
            console::error_1(&format!("Error fetching compilers: {:?}", e).into());
          }
        }
      });
    },
  );

  html! {
    <div class="editor-controls">
      <select>
        { compiler_options.iter().map(|(value, label)| {
          html! { <option value={value.clone()}>{label}</option> }
        }).collect::<Html>() }
      </select>
    </div>
  }
}
  
#[derive(Properties, PartialEq)]
pub struct TextEditorProps {
  #[prop_or_default]
  pub value: String,
}

#[function_component]
fn TextEditor(props: &TextEditorProps) -> Html {
  
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

  let on_run = {
    let app_state = app_state.clone(); 
    Callback::from(move |_e: MouseEvent| {
      let app_state = app_state.clone();
      let code = app_state.code.clone(); 
      spawn_local(async move {        
        let request_body = ApiExecRequest { code: code };
         
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
          Err(e) => {}
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

#[derive(Properties, Clone, PartialEq)]
pub struct AppProviderProps {
  pub children: Children,
}

#[function_component]
fn AppStateProvider(props: &AppProviderProps) -> Html {
  let initial_state = AppState {
    code: String::new(),
    compiler_option: Compiler::default(),
    stdin: String::new(),
    stdout: String::new(),
    stderr: String::new(),
  };

  let app_state = use_reducer(|| initial_state);

  html! {
      <ContextProvider<UseReducerHandle<AppState>> context={app_state}>
          { props.children.clone() }
      </ContextProvider<UseReducerHandle<AppState>>>
  }
}

#[function_component]
fn OutputPanes() -> Html {
   
  let app_state = use_context::<UseReducerHandle<AppState>>()
    .expect("No State found");
  
  html! { 
    <div>
      <TextBox name="stdin"
               placeholder="Standard Input..."
               readonly={true}
               value={app_state.stdin.clone()}/>
      <TextBox name="stdout"
               placeholder="Standard Output..."
               readonly={true}
               value={app_state.stdout.clone()}/>
      <TextBox name="stderr"
               placeholder="Standard Error..."
               readonly={true}
               value={app_state.stderr.clone()}/>   
    </div>
  }

}

#[function_component]
fn App() -> Html {
  html! {
    <AppStateProvider>
      <div>
        <div id="main-container">
          <div id="left">
            <div id="editor" class="container">
              <TextEditor />
            </div>
          </div>
          <div id="right">
            <OutputPanes />
          </div>
        </div>
        <footer>
          <p> {"© 2025 GazBolt"}</p>
        </footer>
      </div>
    </AppStateProvider>
  }
}

fn main() {
  println!("This is main...");
  yew::Renderer::<App>::new().render();
}
