use core::ApiCompilerListView;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::config;
use crate::state::{AppState, AppAction};

#[function_component]
pub fn CompilerDropdown() -> Html {

    let selected_compiler = use_state(|| String::new());
    let app_state = use_context::<UseReducerHandle<AppState>>().expect("No State found");
    let compiler_options = use_state(|| Vec::<(String, String)>::new());
    let options = compiler_options.clone();
    
    use_effect_with((), move |_| {
        let options = options.clone();

        spawn_local(async move {
            match Request::get(&config::create_url("/api/compilers"))
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
                                    .map(|c| {
                                        let c_value = format!("{}|{}", c.name, c.version);
                                        let c_label = format!("{} {}", c.name, c.version);
                                        (c_value, c_label)
                                    })
                                    .collect();
                                options.set(formatted_options);
                            }
                            Err(e) => {
                                console::error_1(
                                    &format!("Error parsing response: {:?}", e).into(),
                                );
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
    });
    
    let on_change = {
        let selected_compiler = selected_compiler.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let select = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            let name_version_str: String = select.value();  
            console::log_1(&format!("Selected compiler: {}", name_version_str).into());

            // Handle state update
            let parts: Vec<&str> = name_version_str.split('|').collect(); 
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let version = parts[1].to_string();
                app_state.dispatch(AppAction::UpdateCompiler(name));            
                app_state.dispatch(AppAction::UpdateVersion(version));
            } else {
                console::error_1(&"Invalid format for compiler value".into());
            }
        })
    };

    html! {
      <div class="compiler-dropdown">
        <select onchange={on_change}>
          <option value="" disabled={true} selected={selected_compiler.is_empty()}>
            { "Select a compiler" }
          </option>
          { compiler_options.iter().map(|(value, label)| {
            html! { 
              <option 
                value={value.clone()} 
                selected={&*selected_compiler == value}
              >
                {label}
              </option> 
            }
          }).collect::<Html>() }
        </select> 
      </div>
    }
}

