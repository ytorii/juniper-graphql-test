#[macro_use]
extern crate diesel;
extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpResponse, HttpServer};

use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;
#[allow(dead_code)]
mod read_request_body;
mod schema;

use graphql_schema::{create_schema, Schema};

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to_async(graphiql)))
    })
    .bind("0.0.0.0:8080")?
    .run()
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://0.0.0.0:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
