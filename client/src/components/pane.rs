use crate::state::AppState;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaneProps {
    name: AttrValue,
    placeholder: AttrValue,
    #[prop_or_default]
    readonly: bool,
    value: AttrValue,
}

#[function_component]
pub fn Pane(props: &PaneProps) -> Html {
    html! {
      <textarea
        name={props.name.clone()}
        placeholder={props.placeholder.clone()}
        readonly={props.readonly}
        value={props.value.clone()}
      />
    }
}

#[function_component]
pub fn OutputPanes() -> Html {
    let app_state = use_context::<UseReducerHandle<AppState>>().expect("No State found");

    html! {
      <div>
        <Pane name="stdin"
              placeholder="Standard Input..."
              readonly={false}
              value={app_state.stdin.clone()}/>
        <Pane name="stdout"
              placeholder="Standard Output..."
              readonly={true}
              value={app_state.stdout.clone()}/>
        <Pane name="stderr"
              placeholder="Standard Error..."
              readonly={true}
              value={app_state.stderr.clone()}/>
      </div>
    }
}
