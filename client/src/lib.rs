mod api;
mod components;
mod context;
mod pages;
mod router;
mod store;

use crate::components::modal::ModalComponent;
use crate::components::{
    alert::{AlertComponent, Props as AlertProps},
    layouts::{user_context_provider::UserContextProvider, Layout},
    spinner::Spinner,
};
use crate::store::{set_open_form, Store};

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component]
pub fn App() -> Html {
    // let (store, _) = use_store::<Store>();
    // let message = store.alert_input.alert_message.clone();
    // let show_alert = store.alert_input.show_alert;

    // let alert_props = AlertProps {
    //     message,
    //     delay_ms: 5000,
    // };

    html! {
        <BrowserRouter>
          <UserContextProvider>
            <CreateFormStory/>
            <Layout>
              <Switch<Route> render={switch} />
            </Layout>
          </UserContextProvider>
        </BrowserRouter>
    }
}

#[function_component(CreateFormStory)]
pub fn view_modal() -> Html {
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
       <ModalComponent>
            <div>{"Hello"}</div>
            <button onclick={handle_open}>
                {"Close"}
            </button>
       </ModalComponent>
    }
}
