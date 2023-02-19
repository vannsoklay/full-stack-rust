use actix_web::web;
use crate::services::story::{ create_story, show_one_story, show_many_story};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(create_story)
        .service(show_one_story)
        .service(show_many_story);

    conf.service(scope);
}
