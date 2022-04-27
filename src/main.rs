//use actix::*;

use actix_web::{web, App, HttpServer};
//use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};


use mongodb::{Client, options::ClientOptions};
use std::sync::*;


mod controllers;


// we use the attribute #[actix_rt::main] to ensure it’s executed with the actix runtime

// creates a MongoDB client that is wrapped in a Mutex for thread safety which is then passed into the app state to be used by our controllers.



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let mut client_options = ClientOptions::parse(
        "mongodb://127.0.0.1:27017/flightdb").await.unwrap();

    client_options.app_name = Some("Flights".to_string());

    let client = web::Data::new(
        Mutex::new(
            Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .route("/flight", web::get().to(controllers::get_flights))
            .route("/flight", web::post().to(controllers::create_flight))
            .route("/flight/{id}", web::get().to(controllers::fetch_one))
            .route("/flight/{id}", web::patch().to(controllers::update_flight))
            .route("/flight/{id}", web::delete().to(controllers::delete_flight))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
