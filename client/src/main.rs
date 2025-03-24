pub mod state;
pub mod compiler;
use state::{AppAction, AppState};
use compiler::Compiler;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::console;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Properties, PartialEq)]
pub struct TextBoxProps {
    name: AttrValue,
    placeholder: AttrValue,
    #[prop_or_default] 
    readonly: bool,  
}

#[function_component]
fn TextBox(props: &TextBoxProps) -> Html { 
    html! {
        <textarea
            name={props.name.clone()}
            placeholder={props.placeholder.clone()}
            readonly={props.readonly}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectorProps {
    pub class: AttrValue,
    pub options: Vec<(String, String)>  // (value, display_text)
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
    let compiler_options: Vec<(String, String)> = Compiler::collect()
        .iter()
        .map(|c| (c.to_string(), c.to_string()))
        .collect();
     
    html! {
        <Selector class="my-selector" options={compiler_options}/>
    } 
}

#[derive(Properties, PartialEq)]
pub struct TextEditorProps {
    #[prop_or_default]
    pub value: String,
}

#[function_component]
fn TextEditor(props: &TextEditorProps) -> Html {
    let app_state = use_context::<UseReducerHandle<AppState>>()
        .expect("No State found");  
    let code = &app_state.code;

    let on_code_change = {
        let app_state = app_state.clone(); // copy to move into closure
         
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>();
            if let Some(input) = input {
                app_state.dispatch(AppAction::UpdateCode(input.value().into()));
            }
        })
    };
   
    let on_click = {
        Callback::from(move |_| { 
            spawn_local(async move {
                match Request::get("http://127.0.0.1:3000/api/compilers")
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.status() {
                            200 => {
                                console::log_1(&"GET received 200".into());
                                match response.json::<serde_json::Value>().await {
                                    Ok(compilers) => {
                                        console::log_1(&compilers.to_string().into());
                                    },
                                    Err(e) => {
                                        console::log_1(&"Failed to deserialize!".into());
                                    }
                                }
                            },
                            404 => {
                                console::log_1(&"GET received 404".into());
                            },
                            _ => {
                                console::log_1(&format!("GET received: {}", response.status()).into());
                            }
                        }
                    },
                    Err(e) => {},
                }

            });
            console::log_1(&"me".into());
        })
    };

    html! {
        <div>
            { "This is the code editor" }
            <EditorControls />
            <textarea 
                width={"300"} 
                height={"500"} 
                rows={"24"}
                value={code.clone()}
                onchange={on_code_change}
            />
            <button onclick={on_click}>
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
                        <TextBox name="stdin"
                                 placeholder="Standard Input..."
                                 readonly={true} />
                        <TextBox name="stdout"
                                 placeholder="Standard Output..."
                                 readonly={true} />
                        <TextBox name="stderr"
                                 placeholder="Standard Error..."
                                 readonly={true} />
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

