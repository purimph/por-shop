use actix_web::web;

// use crate::controllers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("",web::post().to(crate::controllers::user::create_user))
            .route("/{id}",web::get().to(crate::controllers::user::get_user)),
    );
}