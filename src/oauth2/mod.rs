use std::sync::Arc;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use qstring::QString;

use crate::database::model::Client;
use crate::gql::context::Context;
use crate::oauth2::error::OAuth2Error;

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
) -> Result<HttpResponse, OAuth2Error> {
    let conn = context
        .database_pool
        .get()
        .map_err(|_| OAuth2Error::Unknown)?;

    let query_string = req.query_string();
    let queries = QString::from(query_string);

    let client_id = queries
        .get("client_id")
        .ok_or(error::OAuth2Error::ClientIdIsNone)?;
    let client_secret = queries
        .get("client_secret")
        .ok_or(error::OAuth2Error::ClientSecretIsNone)?;

    let da_client: Client = {
        use crate::database::schema::client::dsl::*;
        use diesel::prelude::*;

        client
            .filter(id.eq(client_id.as_bytes().to_vec()))
            .filter(secret.eq(client_secret.to_string()))
            .get_result(&conn)
            .map_err(|_| OAuth2Error::Unknown)?
    };

    Ok(HttpResponse::Ok().body(String::from_utf8(da_client.id).map_err(|_| OAuth2Error::Unknown)?))
}
