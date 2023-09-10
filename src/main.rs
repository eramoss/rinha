mod person;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use person::*;
use routes::*;
use std::env;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/contagem-pessoas", get(|| amount_of_people()))
        .route("/pessoas/:id", get(|| search_person_by_id()))
        .route("/pessoas", post(|person: String| create_person(person)))
        .route(
            "/pessoas",
            get(|query: String| search_people_by_term(query)),
        );

    // run it with hyper on localhost:3000s
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
