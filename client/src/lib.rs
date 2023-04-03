// mod api;
mod api;
mod components;
mod context;
mod pages;
mod router;
mod store;

use crate::components::{
    alert::{AlertComponent, Props as AlertProps},
    layouts::{user_context_provider::UserContextProvider, Layout},
    spinner::Spinner,
};

use crate::store::Store;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component]
pub fn App() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let is_page_loading = store.page_loading.clone();

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };

    html! {
        <BrowserRouter>
          <UserContextProvider>
            <Layout>
              <Switch<Route> render={switch} />
                if show_alert {
                  <AlertComponent
                    message={alert_props.message}
                    delay_ms={alert_props.delay_ms}
                  />
                }
            </Layout>
          </UserContextProvider>
        </BrowserRouter>
    }
}
