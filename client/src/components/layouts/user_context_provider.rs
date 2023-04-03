use crate::api::user::api_user_info;
use yew::prelude::*;
use yew_hooks::prelude::*;
// use crate::error::Error;
// use crate::services::{auth::*, get_token, set_token};
use crate::api::types::CtxUser;
use crate::router;
use yew_router::prelude::use_navigator;
use yewdux::log::info;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(|| CtxUser::default());
    let loading = use_state(|| true);
    let navigator = use_navigator().unwrap();
    {
        let user_ctx = user_ctx.clone();
        let loading = loading.clone();
        use_effect_with_deps(
            move |_| {
                let user_ctx = user_ctx.clone();
                let loading = loading.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    let response = api_user_info().await;
                    match response {
                        Ok(data) => {
                            user_ctx.set(CtxUser {
                                is_authenticated: true,
                                user: data,
                            });
                            loading.set(false);
                        }
                        Err(e) => {
                            info!("error: {:?}", e);
                            navigator.push(&router::Route::Login);
                            loading.set(false);
                        }
                    }
                });
                || ()
            },
            (),
        )
    }
    html! {
        <ContextProvider<UseStateHandle<CtxUser>> context={user_ctx.clone()}>
            if *loading.clone() { 
                <div>{"loading...."}</div>
            } else {
                { for props.children.iter() }
            }
        </ContextProvider<UseStateHandle<CtxUser>>>
    }
}
