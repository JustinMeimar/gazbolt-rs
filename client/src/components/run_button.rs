use core::{ApiExecRequest, ApiExecResponse};
use crate::state::{AppState, AppAction};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use crate::config;

#[function_component]
pub fn RunButton() -> Html {
    
    let app_state = use_context::<UseReducerHandle<AppState>>()
        .expect("No State found");
      
    let on_run = {
        let app_state = app_state.clone();
        Callback::from(move |_e: MouseEvent| {
            let app_state = app_state.clone();
            let code = app_state.code.clone();
            spawn_local(async move {
                let request_body = ApiExecRequest { code }; 
                let api_route = format!("api/run/{}", app_state.selected_compiler);
                match Request::post(&config::create_url(&api_route))
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
                            } } 404 => {
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
      <button onclick={on_run}>
        {"Run"}
      </button>
    }
}

