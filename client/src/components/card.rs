use yew::{function_component, html, Html, Properties, Children};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
   html! {
    <article class="max-w-sm space-y-3 relative">
        { for props.children.iter() }
    </article>
   }
}