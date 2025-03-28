use core::ApiCompilerListView;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[function_component]
pub fn EditorControls() -> Html {
    let compiler_options = use_state(|| Vec::<(String, String)>::new());

    let options = compiler_options.clone();
    use_effect_with((), move |_| {
        let options = options.clone();

        spawn_local(async move {
            match Request::get("http://127.0.0.1:3000/api/compilers")
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
                                    .map(|c| (c.name.clone(), format!("{} {}", c.name, c.version)))
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

    html! {
      <div class="editor-controls">
        <select>
          { compiler_options.iter().map(|(value, label)| {
            html! { <option value={value.clone()}>{label}</option> }
          }).collect::<Html>() }
        </select>
      </div>
    }
}
