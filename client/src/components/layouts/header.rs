use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <div class="fixed w-full top-0">
        <div class="h-16 bg-gray-50">
            <nav class="container-xl px-4 lg:px-0 lg:max-w-screen-lg mx-auto flex justify-between items-center h-full">
                <ul class="flex space-x-3">
                    <li class="text-gray-600">
                        <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                    </li>
                </ul>
                <ul class="space-x-3">
                    <li class="text-gray-600">
                        <Link<Route> to={Route::Login}>{"Login"}</Link<Route>>
                    </li>
                </ul>
                </nav>
            </div>
        </div>
    }
}
