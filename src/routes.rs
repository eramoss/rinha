use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use uuid::Uuid;

use crate::{persistence::Repo, person::Person};

type AppState = Arc<Repo>;

pub async fn search_people_by_term(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<Vec<Person>>), StatusCode> {
    let term = match query.get("t") {
        Some(term) => term,
        None => return Err(StatusCode::UNPROCESSABLE_ENTITY),
    };
    let result = state.select_by_term(term.to_string()).await;

    let people = match result {
        Ok(people) => {
            if people.is_empty() {
                return Err(StatusCode::NOT_FOUND);
            }
            people
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok((StatusCode::OK, Json(people)))
}

pub async fn search_person_by_id(
    State(state): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Person>), StatusCode> {
    let result = state.select_by_id(person_id).await;

    match result {
        Ok(person) => Ok((StatusCode::OK, Json(person))),
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
pub async fn create_person(
    State(state): State<AppState>,
    person: Json<Person>,
) -> Result<(StatusCode, Json<Person>), StatusCode> {
    let result = state.create_new_person(person).await;

    match result {
        Ok(person) => Ok((StatusCode::CREATED, person)),
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
pub async fn amount_of_people(
    State(state): State<AppState>,
) -> Result<(StatusCode, String), StatusCode> {
    let result = state.count_amount_of_person().await;
    match result {
        Ok(count) => Ok((StatusCode::OK, count.to_string())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
