use crate::pages::{home::Home, login::Login, setting::Setting , notfound::NotFound};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/setting")]
    Setting,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Setting => html! { <Setting/> },
        Route::NotFound => html! { <NotFound /> }
    }
}