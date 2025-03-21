use yew::prelude::*;
use web_sys::console;
use serde::{Serialize, Deserialize};

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
    class: AttrValue,
}

#[function_component]
fn Selector(props: &SelectorProps) -> Html {
    html! {
        <select class={props.class.clone()}>
            <option value="1"> {"1"} </option>
            <option value="2"> {"2"} </option>
            <option value="3"> {"3"} </option>
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
fn TextEditor(props: &TextEditorProps) -> Html {
    let onchange = {
        let callback = props.onchange.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlTextAreaElement>();
            if let Some(input) = input {
                console::log_1(&input.value().into());
                console::log_1(&"Hello from Yew".into());
                callback.emit(input.value());
            }
        })
    };

    html! {
        <div>
            { "This is the code editor" }
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

#[function_component]
fn CompilerOptions() -> Html {
    html! {
        <div>
            { "Compiler Options" }
            <Selector class="dropdown"/> 
        </div>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Compilers {
    Generator,
    SCalc,
    VCalc,
    Gazprea,
}

impl Compilers {
    fn default() -> Self {
        Compilers::Generator
    }
}

#[function_component]
fn App() -> Html {
    
    // TODO: Move into struct "AppState" ?
    let code = use_state(|| String::new());
    let compiler_option = use_state(|| Compilers::default());
    let stdin = use_state(|| String::new());
    let stdout = use_state(|| String::new());
    let stderr = use_state(|| String::new());
    
    code.set("me".to_string());

    html! {
        <div>
            <div id="main-contianer">
                <div id="left">
                    <div id="editor" class="container">
                        <CompilerOptions/> 
                        <TextEditor/> 
                        <button action="submit">{"Run"}</button>
                    </div>
                </div>
                <div id="right">
                    <TextBox name="stdin" placeholder="Standard Input..." readonly={true}/>
                    <TextBox name="stdout" placeholder="Standard Output..." readonly={true}/>
                    <TextBox name="stderr" placeholder="Standard Error..." readonly={true}/>
                </div>
            </div>
            <footer>
                <p> {"© 2025 GazBolt"}</p>
            </footer>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

