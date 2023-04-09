
use yew::prelude::*;

#[function_component(Setting)]
pub fn setting() -> Html {
    html! {
        <div>
            <p class={classes!(String::from("text-red-600"))}>{"Setting Profile"}</p>
        </div>
    }
}