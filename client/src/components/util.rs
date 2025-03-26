use yew::prelude::*;

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
