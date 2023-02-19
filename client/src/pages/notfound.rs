use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div>
            <p class={classes!(String::from("text-red-600"))}>{"Not Found!"}</p>
        </div>
    }
}