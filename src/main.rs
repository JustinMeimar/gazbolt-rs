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

#[derive(Properties, PartialEq)]
pub struct TextEditorProps {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub onchange: Callback<String>,
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

#[function_component]
fn TextEditor(props: &TextEditorProps) -> Html {
    let app_state = use_context::<UseReducerHandle<AppState>>().expect("No State found"); 
    
    let onchange = {
        let callback = props.onchange.clone();
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
            <EditorControls />
            <textarea 
                width={"300"} 
                height={"500"} 
                rows={"24"}
                value={props.value.clone()}
                onchange={onchange}
            />
        </div>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Compiler {
    Generator,
    SCalc,
    VCalc,
    Gazprea,
}

impl Compiler {
    fn default() -> Self {
        Compiler::Generator
    }
    fn collect() -> Vec<Self> {
        return vec![
            Compiler::Generator,
            Compiler::SCalc,
            Compiler::VCalc,
            Compiler::Gazprea
        ]
    }
}


impl fmt::Display for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Compiler::Generator => write!(f, "Generator"),
            Compiler::SCalc => write!(f, "SCalc"),
            Compiler::VCalc => write!(f, "VCalc"),
            Compiler::Gazprea => write!(f, "Gazprea"),
        }
    }
}


use std::rc::Rc;

#[derive(PartialEq, Clone)]
struct AppState {
    code: String,
    compiler_option: Compiler,
    stdin: String,
    stdout: String,
    stderr: String,
}

enum AppAction {
    UpdateCode(String),
    UpdateCompiler(Compiler),
    UpdateStdin(String),
    UpdateStdout(String),
    UpdateStderr(String),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        
        match action {
            AppAction::UpdateCode(code) => next_state.code = code,
            AppAction::UpdateCompiler(compiler) => next_state.compiler_option = compiler,
            AppAction::UpdateStdin(stdin) => next_state.stdin = stdin,
            AppAction::UpdateStdout(stdout) => next_state.stdout = stdout,
            AppAction::UpdateStderr(stderr) => next_state.stderr = stderr,
        }
        
        next_state.into()
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
                            // <CompilerOptions /> 
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
    yew::Renderer::<App>::new().render();
}

