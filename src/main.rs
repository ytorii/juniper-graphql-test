#[macro_use]
extern crate diesel;
extern crate juniper;
extern crate serde_derive;
extern crate dotenv;

use std::io;
use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpResponse, HttpServer};

use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;
#[allow(dead_code)]
mod schema;

use dotenv::dotenv;
use graphql_schema::{create_schema, Schema};

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    dotenv().ok();
    let allowed_origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be set");
    let graphiql_origin = env::var("GRAPHIQL_ORIGIN").expect("GRAPHIQL_ORIGIN must be set");

    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&allowed_origin)
                    .allowed_origin(&graphiql_origin)
                    .allowed_methods(vec!["GET", "POST", "OPTION"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
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
    dotenv().ok();
    let graphql_url = env::var("GRAPHQL_URL").expect("GRAPHQL_URL must be set");
    let html = graphiql_source(&graphql_url);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
