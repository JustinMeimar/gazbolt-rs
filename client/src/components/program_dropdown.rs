use yew::prelude::*;

#[function_component]
pub fn ProgramDropdown() -> Html {
    let program_options = use_state(|| vec!["loop.in".to_string(), "vec.in".to_string()]);
     
    html! {
      <div class="program-dropdown">
          <select>
          { program_options.iter().map(|program| {
            html! { <option value={program.clone()}>{program}</option> }
          }).collect::<Html>() }
        </select> 
      </div>
    }
}

