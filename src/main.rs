use yew::prelude::*;

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
        >
        </textarea>
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectorProps {
    class: AttrValue,
}

#[function_component]
fn Selector(props: &SelectorProps) -> Html {
    html! {
        <select
            class={props.class.clone()}
        >
            <option value="1"> {"1"} </option>
            <option value="2"> {"2"} </option>
            <option value="3"> {"3"} </option>
        </select>
    }
}

#[function_component]
fn TextEditor() -> Html {
    html! {
        <div>
            { "This is the code editor" }
            <textarea width={"300"} height={"500"} rows={"24"}>
            </textarea>
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

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <div id="main-contianer">
                <div id="left">
                    <div id="editor" class="container">
                        <CompilerOptions/> 
                        <TextEditor/> 
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

