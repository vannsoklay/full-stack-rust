use std::fmt;
use std::ops::Deref;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::types::{User, UserLoginResponse};
use crate::api::user::api_user_info;
use crate::router::Route;

/// State handle for the [`use_user_context`] hook.
pub struct UseUserContextHandle {
    inner: UseStateHandle<User>,
    navigator: Navigator,
}

impl UseUserContextHandle {
    pub async fn login(&self, value: UserLoginResponse) {
        if !value.access_token.is_empty() {
            let user = api_user_info().await;
            match user {
                Ok(data) => {
                    self.inner.set(data);
                    self.navigator.push(&Route::Home);
                }
                Err(_) => self.logout(),
            }
        }
    }

    pub fn logout(&self) {
        self.inner.set(User::default());
        self.navigator.push(&Route::Login);
    }
}

impl Deref for UseUserContextHandle {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            navigator: self.navigator.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

/// This hook is used to manage user context.
#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<User>>().unwrap();
    let navigator = use_navigator().unwrap();

    UseUserContextHandle { inner, navigator }
}
