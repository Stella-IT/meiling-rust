#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::io::{Error as StdError, ErrorKind as StdErrorKind};
use std::sync::Arc;

use actix_web::{App, Error, get, HttpResponse, HttpServer, post, Responder, web};
use juniper::http::{GraphQLRequest, playground::playground_source};

use crate::config::{Config, ConfigError};
use crate::database::{
    connection::{establish_diesel_connection, establish_r2d2_connection},
    error::DatabaseError,
};
use crate::database::connection::build_pool;
use crate::gql::context::Context;
use crate::gql::schema::{create_schema, Schema};

mod config;
mod database;
mod gql;
mod meiling;

#[get("/graphql")]
async fn playground() -> HttpResponse {
    let html = playground_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(
    st: web::Data<Arc<Schema>>,
    context: web::Data<Arc<Context>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hey, This is meiling API Server! We don't have front-end yet")
}

embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load().map_err(|e| match e {
        ConfigError::MissingKey(_) => StdError::new(StdErrorKind::NotFound, e.to_string()),
        ConfigError::InvalidFormat(_) => StdError::new(StdErrorKind::InvalidInput, e.to_string()),
    })?;

    let connection = establish_diesel_connection(&config.database).expect("Database Error");

    embedded_migrations::run(&connection)
        .map_err(|e| DatabaseError::MigrationError(e.to_string()))
        .unwrap();

    let manager = establish_r2d2_connection(&config.database);

    let pool = build_pool(manager).map_err(|e| match e {
        DatabaseError::ConnectionError(_) => {
            StdError::new(StdErrorKind::NotConnected, e.to_string())
        }
        DatabaseError::MigrationError(_) => {
            StdError::new(StdErrorKind::ConnectionRefused, e.to_string())
        }
        DatabaseError::PoolError(_) => {
            StdError::new(StdErrorKind::ConnectionRefused, e.to_string())
        }
    })?;

    let context = Arc::new(Context::new(pool, config.clone()));

    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(context.clone())
            .service(root)
            .service(playground)
            .service(graphql)
            .service(
                web::scope("oauth2")
                    .service(oauth2::auth_redirect)
                    .service(oauth2::get_token)
            )
    })
    .bind(config.bind_address)?
    .run()
    .await
}
