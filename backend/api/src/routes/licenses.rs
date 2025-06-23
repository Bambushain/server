use actix_web::{get, Responder};
use bamboo_common::backend::response::list;
use bamboo_common::core::entities::get_dependencies;

#[get("/api/licenses")]
pub async fn get_licenses() -> impl Responder {
    list!(get_dependencies())
}
