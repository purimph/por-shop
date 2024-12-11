use crate::controllers::product::{create_product, delete_product, get_product, get_products, update_product};
use actix_web::web;

pub fn configure_product(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_products))
            .route("/{id}", web::get().to(get_product))
            .route("", web::post().to(create_product))
            .route("/{id}", web::delete().to(delete_product))
            .route("/{id}", web::put().to(update_product)),
    );
}
