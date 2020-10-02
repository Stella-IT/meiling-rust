use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use std::sync::Arc;
use crate::gql::schema::Schema;
use crate::gql::context::Context;
use qstring::QString;
use crate::database::model::Client;
use std::error::Error;

#[get("/auth")]
pub async fn auth_redirect(req: HttpRequest) -> impl Responder {
    const ACCOUNT_SERVER: &str = "https://accounts.stella-it.com/auth";
    let query_string: String = req.query_string().to_string();

    HttpResponse::TemporaryRedirect().header("Location", format!("{}?{}", ACCOUNT_SERVER, query_string)).body("")
}

#[get("/token")]
async fn get_token(
    st: web::Data<Arc<Schema>>,
    context: web::Data<Arc<Context>>,
    req: HttpRequest,
) -> Result<HttpResponse, Box<dyn Error>> {
    use crate::database::schema::client::dsl::*;
    use diesel::prelude::*;

    let conn = context.database_pool.get()?;

    let query_string = req.query_string();
    let queries = QString::from(query_string);

    let da_client = {
        let client_id = queries.get("client_id");

        client.filter(id.eq(client_id.into())).get_result(&conn)?
    };

    Ok(HttpResponse::Ok().body(da_client.id.into()))
}