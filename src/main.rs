mod persistence;
mod person;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use persistence::Repo;
use routes::*;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let url_api = env::var("URL_API").unwrap_or("0.0.0.0:80".to_string());
    let database_url = env::var("DATABASE_URL").expect("expected URL of the database");

    let pool = Repo::new(database_url).await;
    let app_state = Arc::new(pool);

    // build our application with a single route
    let app = Router::new()
        .route("/contagem-pessoas", get(amount_of_people))
        .route("/pessoas/:id", get(search_person_by_id))
        .route("/pessoas", post(create_person))
        .route("/pessoas", get(search_people_by_term))
        .with_state(app_state);

    axum::Server::bind(&url_api.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
