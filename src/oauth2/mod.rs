use std::sync::Arc;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use qstring::QString;

use crate::gql::context::Context;
use crate::meiling::{oauth2, objects::oauth2 as oauth2_objects};

pub mod error;

#[get("/auth")]
pub async fn auth_redirect(req: HttpRequest) -> impl Responder {
    const ACCOUNT_SERVER: &str = "https://accounts.stella-it.com/auth";
    let query_string: String = req.query_string().to_string();

    HttpResponse::TemporaryRedirect()
        .header("Location", format!("{}?{}", ACCOUNT_SERVER, query_string))
        .body("")
}

#[get("/token")]
async fn get_token(
    context: web::Data<Arc<Context>>,
    req: HttpRequest,
) -> Result<HttpResponse, error::OAuth2Error> {
    let query_string = req.query_string();
    let queries = QString::from(query_string);

    let client_id = queries
        .get("client_id")
        .ok_or(error::OAuth2Error::ClientIdIsNone)?
        .to_string();
    let client_secret = queries
        .get("client_secret")
        .ok_or(error::OAuth2Error::ClientSecretIsNone)?
        .to_string();

    let conn = context
        .database_pool
        .get()
        .map_err(|_| error::OAuth2Error::Unknown)?;

    let token = oauth2::get_token(
        &conn,
        oauth2_objects::NewTokenRequest {
            client_id,
            client_secret,
        },
    )
    .map_err(|e| error::OAuth2Error::from(e))?;

    Ok(HttpResponse::Ok()
        .body(serde_json::to_string(&token).map_err(|e| error::OAuth2Error::Unknown)?))
}
