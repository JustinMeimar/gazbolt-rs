use core::{ApiCompilerListView, ApiExecRequest, ApiExecResponse};
use crate::state::{AppState, AppAction};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[function_component]
pub fn EditorControls() -> Html {
    
    let app_state = use_context::<UseReducerHandle<AppState>>().expect("No State found");
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
    
    let on_run = {
        let app_state = app_state.clone();
        Callback::from(move |_e: MouseEvent| {
            let app_state = app_state.clone();
            let code = app_state.code.clone();
            spawn_local(async move {
                let request_body = ApiExecRequest { code };

                match Request::post("http://127.0.0.1:3000/api/run/gcc")
                    .header("Content-Type", "application/json")
                    .json(&request_body)
                    .expect("Faield to serialize request body")
                    .send()
                    .await
                {
                    Ok(response) => match response.status() {
                        200 => {
                            console::log_1(&"POST received 200".into());
                            match response.json::<ApiExecResponse>().await {
                                Ok(exec_response) => {
                                    app_state
                                        .dispatch(AppAction::UpdateStdout(exec_response.stdout));
                                    app_state
                                        .dispatch(AppAction::UpdateStderr(exec_response.stderr));
                                }
                                Err(e) => {
                                    console::log_1(&format!("Failed to deserialize: {}", e).into());
                                }
                            }
                        }
                        404 => {
                            console::log_1(&"GET received 404".into());
                        }
                        _ => {
                            console::log_1(&format!("GET received: {}", response.status()).into());
                        }
                    },
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }
            });
        })
    };

    html! {
      <div class="editor-controls">
        <select>
          { compiler_options.iter().map(|(value, label)| {
            html! { <option value={value.clone()}>{label}</option> }
          }).collect::<Html>() }
        </select>
        <button onclick={on_run}>
          {"Run"}
        </button>
      </div>
    }
}
