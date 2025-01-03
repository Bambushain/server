use actix_web::http::header;
use actix_web::middleware::Next;
use actix_web::{body, dev, Error, HttpMessage, HttpResponse};
use bamboo_common::backend::services::DbConnection;

use bamboo_common::backend::actix::cookie;
use bamboo_common::backend::actix::middleware::{
    get_user_and_token_by_cookie, AuthenticationState,
};

pub async fn authenticate_user(
    db: DbConnection,
    auth_cookie: Option<cookie::BambooAuthCookie>,
    req: dev::ServiceRequest,
    next: Next<impl body::MessageBody>,
) -> Result<dev::ServiceResponse<body::EitherBody<impl body::MessageBody, ()>>, Error> {
    if let Ok((token, user)) = get_user_and_token_by_cookie(&db, auth_cookie).await {
        req.extensions_mut()
            .insert(AuthenticationState { token, user });

        Ok(next.call(req).await?.map_into_left_body())
    } else {
        let req = req.request().to_owned();

        Ok(dev::ServiceResponse::new(
            req,
            HttpResponse::Found()
                .insert_header((header::LOCATION, "/authentication"))
                .message_body(())?,
        )
        .map_into_right_body())
    }
}

#[allow(dead_code)]
pub type AuthState = actix_web::web::ReqData<AuthenticationState>;
