use super::spinner::Spinner;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub loading: bool,
    pub btn_color: Option<String>,
    pub text_color: Option<String>,
    pub children: Children,
}

#[function_component(LoadingButton)]
pub fn loading_button_component(props: &Props) -> Html {
    let text_color = props
        .text_color
        .clone()
        .unwrap_or_else(|| "text-white".to_string());
    let btn_color = props
        .btn_color
        .clone()
        .unwrap_or_else(|| "bg-ct-yellow-600".to_string());

    html! {
    <button
      type="submit"
      class={format!(
        "bg-blue-500 hover:bg-blue-600 text-white w-full font-bold py-2 px-4 rounded-sm text-base {}",
         if props.loading {"bg-[#ccc]"} else {btn_color.as_str()}
      )}
    >
      if props.loading {
        <div class="flex items-center gap-3">
          <Spinner />
          <span class="text-slate-500 inline-block">{"Loading..."}</span>
        </div>
      }else{
        <span class={text_color.to_owned()}>{props.children.clone()}</span>
      }
    </button>
    }
}
