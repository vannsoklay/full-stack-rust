use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user::api_login_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::use_user_context;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]

struct LoginUserSchema {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component]
pub fn Login() -> Html {
    let ctx_user = use_user_context();
    // let (store, dispatch) = use_store::<Store>();
    // let user = store.auth_user.clone();
    let form = use_state(|| LoginUserSchema::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "email" => data.email = value,
                "password" => data.password = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_email_input = get_input_callback("email", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());
    let cloned_ctx = ctx_user.clone();

    let on_submit = {
        let cloned_form = form.clone();
        // let cloned_ctx = ctx_user.clone();
        let cloned_validation_errors = validation_errors.clone();
        // let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();
        
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            // let dispatch = store_dispatch.clone();
            let cloned_ctx = ctx_user.clone();
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();

            let email_input_ref = cloned_email_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        // set_page_loading(true, dispatch.clone());

                        let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                        // email_input.set_value("");
                        // password_input.set_value("");

                        let form_json = serde_json::to_string(&form_data).unwrap();
                        let res = api_login_user(&form_json).await;
                        match res {
                            Ok(data) => {
                                cloned_ctx.login(data).await;
                                // set_page_loading(false, dispatch);
                                // navigator.push(&router::Route::Home);
                            }
                            Err(e) => {
                                // cloned_ctx.login(data).await;
                                // set_page_loading(false, dispatch.clone());
                                // set_show_alert(e.to_string(), dispatch);
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
         <div class="bg-grey-lighter font-sans">
            if cloned_ctx.is_authenticated() {
                <ul class="space-x-3">
                    <li class="text-gray-600">
                       {"Permission Page"}
                    </li>
                </ul>
            } else {
                <main class="container mx-auto flex justify-center items-center">
                    <div class="w-1/3">
                        <div class="font-hairline flex justify-center w-full">
                            <h1 class="font-semibold text-2xl text-gray-600">{"Sign In"}</h1>
                        </div>
                        <div class="p-8 bg-white mb-4">
                            <form onsubmit={on_submit}>
                            <div class="mb-4">
                                <FormInput label="Email" name="email" input_type="email" input_ref={email_input_ref} handle_onchange={handle_email_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}/>
                            </div>
                            <div class="mb-4">
                                <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}/>                        
                            </div>
                            <div class="flex items-center justify-between mt-6">
                                <LoadingButton
                                    loading={false}
                                >
                                {"Sign In"}
                                </LoadingButton>
                            </div>
                            </form>
                        </div>
                    </div>
                </main>
            }
        </div>
    }
}
