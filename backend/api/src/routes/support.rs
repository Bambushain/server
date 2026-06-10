use actix_web::{post, web};
use bamboo_common::backend::mailing;
use bamboo_common::backend::response::*;
use bamboo_common::core::entities::SupportRequest;
use bamboo_common::core::error::*;

use bamboo_common::backend::actix::middleware::{authenticate, Authentication};
use bamboo_common::backend::services::DbConnection;

#[post("/api/support", wrap = "authenticate!()")]
pub async fn send_support_request(
    authentication: Authentication,
    body: Option<web::Json<SupportRequest>>,
    db: DbConnection,
) -> BambooApiResponseResult {
    let body = check_missing_fields!(body, "support")?.into_inner();

    mailing::send_support_request(
        body.message,
        body.subject,
        authentication.user.email.clone(),
        &db,
    )
        .await;

    Ok(no_content!())
}
