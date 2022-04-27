use actix_web::{web, HttpResponse, Responder};
use bson::{doc, oid};
use futures::stream::StreamExt;
use mongodb::{options::FindOptions, Client};
use serde::{Deserialize, Serialize};
use std::sync::*;

#[derive(Serialize, Deserialize)]
pub struct Flight {
    pub content: String,
    pub is_done: bool,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

const MONGO_DB: &'static str = "flightdb";
const MONGO_COLLECTION: &'static str = "flightdb";

/*
We imported the needed modules,
created two structs Flight and Response and two const variables,

The Flight struct is responsible for how model data would be inputted
into the database,

The Response handles how response messages would be sent back on an endpoint.

The MONGO_DB and MONGOCOLLECTION holds the constant strings of our
database name and collection name.
*/

/*
These are the handlers for each route,
they are each asynchronous functions that return a Responder trait
provided by actix-web
*/

/*
pub trait Responder {
    type Item: Into<Reply>;
    type Error: Into<Error>;
    fn respond_to(self, req: HttpRequest) -> Result<Self::Item, Self::Error>;
}
*/

pub async fn get_flights(data: web::Data<Mutex<Client>>) -> impl Responder {
    let flight_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION);

    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! { "_id": -1 }).build();

    let mut cursor = flight_collection.find(filter, find_options).await.unwrap();

    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)

    /*
        format!("Fetch all flights");
        HttpResponse::Ok()
            .body("Fetch all flights")
    */
}

pub async fn create_flight(
    data: web::Data<Mutex<Client>>,
    flight: web::Json<Flight>,
) -> impl Responder {
    let flight_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION);

    match flight_collection
        .insert_one(
            doc! {"content": &flight.content,
            "is_done": &flight.is_done},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                println!("New data inserted with id {}", new_id);
            }
            let response = Response {
                message: "Successful".to_string(),
            };
            return HttpResponse::Created().json(response);
        }
        Err(err) => {
            println!("Failed! {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }

    /*
        format!("Create a new flight");
        HttpResponse::Ok()
            .body("Create a new flight")
    */
}

pub async fn fetch_one(
    data: web::Data<Mutex<Client>>,
    flight_id: web::Path<String>,
) -> impl Responder {
    let flight_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION);

    let filter = doc! {
    "_id": oid::ObjectId:with_string(&flight_id.to_string()).unwrap() };
    let obj = flight_collection.find_one(filter, None).await.unwrap();

    return HttpResponse::Ok().json(obj);

    /*
        format!("Fetch one flight data");
        HttpResponse::Ok()
            .body("Fetch one flight data")
    */
}

pub async fn update_flight(
    data: web::Data<Mutex<Client>>,
    flight_id: web::Path<String>,
    flight: web::Json<Flight>,
) -> impl Responder {
    let flight_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION);
    let filter = doc! {
    "_id": oid::ObjectId::with_string(&flight_id.to_string()).unwrap() };

    let data = doc! {
    "$set": { "content": &flight.content,
              "is_done": &flight.is_done } };

    flight_collection
        .update_one(filter, data, None)
        .await
        .unwrap();

    let response = Response {
        message: "Updated Successfully".to_string(),
    };

    return HttpResponse::Ok().json(response);

    /*
        format!("Update a flight");
        HttpResponse::Ok()
            .body("Update a flight")
    */
}

pub async fn delete_flight(
    data: web::Data<Mutex<Client>>,
    flight_id: web::Path<String>,
) -> impl Responder {
    let flight_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION);

    let filter = doc! {
    "_id": oid::ObjectId::with_string(&flight_id.to_string()).unwrap() };

    flight_collection.delete_one(filter, None).await.unwrap();

    return HttpResponse::NoContent();

    /*
        format!("Delete a flight");
        HttpResponse::Ok()
            .body("Delete a flight")
    */
}
