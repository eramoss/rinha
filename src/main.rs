mod person;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use person::Person;
use routes::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let people: HashMap<Uuid, Person> = HashMap::new();
    let app_state = Arc::new(Mutex::new(people));

    // build our application with a single route
    let app = Router::new()
        .route("/contagem-pessoas", get(amount_of_people))
        .route("/pessoas/:id", get(search_person_by_id))
        .route("/pessoas", post(create_person))
        .route("/pessoas", get(search_people_by_term))
        .with_state(app_state);

    // run it with hyper on localhost:3000s
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
