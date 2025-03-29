//===----------------------------------------------------------------------===//
// ~ file: main.rs
// ~ author: Justin Meimar
// ~ date: march 25th 2025
// ~ desc: Entry point for the Yew frontend application.
//===----------------------------------------------------------------------===//

pub mod components;
pub mod state;
use components::pane::OutputPanes;
use components::text_editor::TextEditor;
use state::AppState;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AppProviderProps {
    pub children: Children,
}

#[function_component]
fn AppStateProvider(props: &AppProviderProps) -> Html {
    let initial_state = AppState {
        code: String::new(),
        compiler_option: "None".to_string(), // initialized on start
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
    yew::Renderer::<App>::new().render();
}

