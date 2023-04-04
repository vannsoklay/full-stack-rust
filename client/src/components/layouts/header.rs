use crate::{api::user::api_logout_user, router::Route};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::context::use_user_context;

#[function_component]
pub fn Header() -> Html {
    let user = use_user_context();
    let cloned_ctx = user.clone();
    let handle_logout = {
        Callback::from(move |_: MouseEvent| {
            let cloned_ctx = user.clone();
            spawn_local(async move {
                let res = api_logout_user().await;
                match res {
                    Ok(_) => {
                        cloned_ctx.logout();
                    }
                    Err(e) => {}
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
                if cloned_ctx.is_authenticated() {
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
