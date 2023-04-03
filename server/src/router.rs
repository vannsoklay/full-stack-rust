use actix_web::web;
use crate::security::{register_user_handler,login_user_handler,get_me_handler, logout_handler,refresh_access_token_handler};
use crate::api::story::{get_story, create_story, get_story_all};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(register_user_handler)
        .service(login_user_handler)
        .service(refresh_access_token_handler)
        .service(get_me_handler)
        .service(logout_handler)
        .service(create_story)
        .service(get_story)
        .service(get_story_all);

    conf.service(scope);
}