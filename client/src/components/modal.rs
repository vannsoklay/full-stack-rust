use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::Store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(ModalComponent)]
pub fn modal_component(props: &Props) -> Html {
    let (store, _) = use_store::<Store>();
    let open_form = store.modal_form;

    html! {
        <div class={format!("overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 md:inset-0 h-full md:h-full bg-gray-400 bg-opacity-30 flex justify-center items-center {}", if !open_form {"hidden"} else {"none"}) } id="popup-modal">
            <div class="relative -top-40 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
                {props.children.clone()}
            </div>
        </div>
    }
}
