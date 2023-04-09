use crate::store::{set_open_form, Store};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Home)]
pub fn home() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let open = store.modal_form;
    let handle_open = {
        let store_dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            let cloned_dispatch = store_dispatch.clone();
            set_open_form(!open, cloned_dispatch.clone());
        })
    };
    html! {
        <div>
            <button onclick={handle_open}>
                {"Open"}
            </button>
        </div>
    }
}