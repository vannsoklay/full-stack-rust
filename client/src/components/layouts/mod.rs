mod header;

use header::Header;
use yew::{function_component, html, Html, Properties, Children};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}


#[function_component(Layout)]
pub fn main_layout(props: &Props) -> Html {
    html! {
        <div class="relative">
            <Header/>
            <div class="container-xl px-4 lg:px-0 lg:max-w-screen-lg mx-auto mt-24">
                { for props.children.iter() }
            </div>
        </div>
    }
}