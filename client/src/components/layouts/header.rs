use crate::{
    api::user::api_logout_user,
    router::{self, Route},
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::{prelude::*, log::info};

use crate::context::use_user_context;

#[function_component]
pub fn Header() -> Html {
    let ctx_user = use_user_context();
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    info!("user {:?}", ctx_user.user);
    let handle_logout = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let dispatch = store_dispatch.clone();
            let navigator = cloned_navigator.clone();
            spawn_local(async move {
                set_page_loading(true, dispatch.clone());
                let res = api_logout_user().await;
                match res {
                    Ok(_) => {
                        set_page_loading(false, dispatch.clone());
                        set_auth_user(None, dispatch.clone());
                        set_show_alert("Logged out successfully".to_string(), dispatch);
                        navigator.push(&router::Route::Login);
                    }
                    Err(e) => {
                        set_show_alert(e.to_string(), dispatch.clone());
                        set_page_loading(false, dispatch);
                    }
                };
            });
        })
    };
    html! {
        <div class="fixed w-full top-0">
        <div class="h-16 bg-gray-50">
            <nav class="container-xl px-4 lg:px-0 lg:max-w-screen-lg mx-auto flex justify-between items-center h-full">
                <ul class="flex space-x-3">
                    <li class="text-gray-600">
                        <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                    </li>
                </ul>
                if ctx_user.is_authenticated {
                     <ul class="space-x-3">
                        <li class="text-gray-600">
                            {"Profile"}
                        </li>
                    </ul>
                    <ul class="space-x-3">
                        <li onclick={handle_logout} class="text-gray-600">
                            {"Logout"}
                        </li>
                    </ul>

                } else {
                    <ul class="space-x-3">
                        <li class="text-gray-600">
                            <Link<Route> to={Route::Login}>{"Login"}</Link<Route>>
                        </li>
                    </ul>
                }
                </nav>
            </div>
        </div>
    }
}
