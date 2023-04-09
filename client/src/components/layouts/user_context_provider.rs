use crate::api::types::User;
use crate::api::user::{api_refresh_token, api_user_info};
use crate::router;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::log::info;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user = use_state(|| User::default());
    let loading = use_state(|| true);
    let navigator = use_navigator().unwrap();
    {
        let user = user.clone();
        let loading = loading.clone();
        use_effect_with_deps(
            move |_| {
                let user = user.clone();
                let loading = loading.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    let response = api_user_info().await;
                    match response {
                        Ok(data) => {
                            user.set(data);
                            loading.set(false);
                        }
                        Err(_) => {
                            let response = api_refresh_token().await;
                            match response {
                                Ok(_) => {
                                    loading.set(false);
                                }
                                Err(_) => {
                                    info!("helllo");
                                    navigator.push(&router::Route::Login);
                                    loading.set(false);
                                }
                            }
                        }
                    }
                });
                || ()
            },
            (),
        )
    }
    html! {
        <ContextProvider<UseStateHandle<User>> context={user.clone()}>
            if *loading.clone() {
                <div class="h-screen flex justify-center items-center">{"loading...."}</div>
            } else {
                { for props.children.iter() }
            }
        </ContextProvider<UseStateHandle<User>>>
    }
}
