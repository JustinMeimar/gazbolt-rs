pub mod state;
pub mod compiler;
use state::{AppAction, AppState};
use compiler::Compiler;
use yew::prelude::*;
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
    
    let on_code_change = {
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>();
            if let Some(input) = input {
                console::log_1(&input.value().into());
                eprintln!("ERROR? Testing");
                app_state.dispatch(
                    AppAction::UpdateCode(
                        input.value().into()
                    )
                );
            }
        })
    };

    let on_code_run = {
        Callback::from(move |e: Event| {
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
                value={props.value.clone()}
                onchange={on_code_change}
            />
            <button onchange={on_code_run}>
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

