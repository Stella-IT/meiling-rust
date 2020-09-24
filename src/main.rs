use std::sync::Arc;

use actix_web::{App, Error, get, HttpResponse, HttpServer, post, Responder, web};
use juniper::http::{GraphQLRequest, playground::playground_source};

use crate::schema::{create_schema, Schema};

mod schema;

#[get("/graphql")]
async fn graphiql() -> HttpResponse {
    let html = playground_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

#[post("/graphql")]
async fn graphql(st: web::Data<Arc<Schema>>, data: web::Json<GraphQLRequest>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(user))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
        .data(schema.clone())
        .service(hello)
        .service(echo)
        .service(graphiql)
        .service(graphql)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
