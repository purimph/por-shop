use crate::controllers::product::{create_product, get_product, get_products};
use actix_web::web;

pub fn configure_product(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_products))
            .route("/{id}", web::get().to(get_product))
            .route("", web::post().to(create_product)),
    );
}
