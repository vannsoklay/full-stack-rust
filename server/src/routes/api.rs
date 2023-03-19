use actix_web::web;
use crate::services::story::{ create_story, find_story, find_all_story};
use crate::security::auth::{register_user_handler,login_user_handler,get_me_handler, logout_handler};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(register_user_handler)
        .service(login_user_handler)
        .service(get_me_handler)
        .service(logout_handler)
        .service(create_story)
        .service(find_story)
        .service(find_all_story);

    conf.service(scope);
}
