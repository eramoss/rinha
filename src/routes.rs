use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::person::Person;

type AppState = Arc<Mutex<HashMap<Uuid, Person>>>;

pub async fn search_people_by_term(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(StatusCode, String), StatusCode> {
    let mut people_response: Vec<Person> = Vec::new();
    for (_, person) in state.lock().await.iter() {
        let json_person = json!(&person).to_string();
        match query.get("t") {
            Some(query) => {
                if json_person.contains(query) {
                    people_response.push(person.clone());
                }
            }
            None => return Err(StatusCode::UNPROCESSABLE_ENTITY),
        };
    }
    if people_response.len() == 0 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    Ok((StatusCode::OK, json!(people_response).to_string()))
}

pub async fn search_person_by_id(
    State(state): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> Result<(StatusCode, String), StatusCode> {
    match state.lock().await.get(&person_id) {
        Some(person) => Ok((StatusCode::OK, json!(person).to_string())),
        None => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
pub async fn create_person(
    State(state): State<AppState>,
    person: String,
) -> Result<(StatusCode, String), StatusCode> {
    match Person::deserialize_from_string(person) {
        Ok(person) => {
            state.lock().await.insert(person.id, person.clone());
            Ok((StatusCode::OK, json!(person).to_string()))
        }
        _ => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
pub async fn amount_of_people(State(state): State<AppState>) -> (StatusCode, String) {
    (StatusCode::OK, json!(state.lock().await.len()).to_string())
}

#[cfg(test)]
mod endpoints_tests {
    use axum::extract::{Path, State};
    use std::sync::Arc;
    use time::macros::date;

    use super::*;
    use crate::{person::Person, routes::search_person_by_id};

    #[tokio::test]
    async fn search_people_by_term_contain_term() {
        let repo = initialize_mock_repo().await; // init with person Roberto
        let mut query: HashMap<String, String> = HashMap::new();
        query.insert("t".to_string(), "berto".to_string());
        let (_, people_as_json) = search_people_by_term(repo, Query(query)).await.unwrap();

        assert!(people_as_json.contains("berto"))
    }

    #[tokio::test]
    async fn search_person_by_id_test() {
        let State(repo) = initialize_mock_repo().await;
        let first_person = get_first_elem_of_repo(&repo).await.unwrap();
        let (_, person_from_fn) = search_person_by_id(State(repo), Path(first_person.id))
            .await
            .unwrap();

        let person_from_fn: Person = Person::deserialize_from_string(person_from_fn).unwrap();
        assert_eq!(first_person, person_from_fn);
    }
    #[tokio::test]
    async fn create_valid_person() {
        let repo: AppState = Arc::new(Mutex::new(HashMap::new()));
        let person = Person::new(
            "John".to_string(),
            "Doe".to_string(),
            date!(1999 - 9 - 9),
            vec!["c".to_string(), "c++".to_string()],
        );
        let person_as_json = json!(person).to_string();
        let _ = create_person(State(repo.clone()), person_as_json).await;

        assert_eq!(repo.lock().await.len(), 1);
    }

    #[tokio::test]
    async fn create_invalid_person() {
        let repo: AppState = Arc::new(Mutex::new(HashMap::new()));
        let person = Person::new(
            "John".to_string(),
            "name with more than 30 characters is not permitted".to_string(),
            date!(1999 - 9 - 9),
            vec!["c".to_string(), "c++".to_string()],
        );

        let result = create_person(State(repo), json!(person).to_string()).await;

        assert_eq!(result, Err(StatusCode::UNPROCESSABLE_ENTITY))
    }

    #[tokio::test]
    async fn count_amount_of_person() {
        let repo: AppState = Arc::new(Mutex::new(HashMap::new()));
        let person = Person::new(
            "John".to_string(),
            "name with more than 30 characters is not permitted".to_string(),
            date!(1999 - 9 - 9),
            vec!["c".to_string(), "c++".to_string()],
        );
        repo.lock().await.insert(person.id, person);

        let (_, amount_people) = amount_of_people(State(repo)).await;

        assert_eq!(1, amount_people.parse::<usize>().unwrap())
    }

    #[tokio::test]
    async fn response_should_be_422_in_get_by_id() {
        let State(repo) = initialize_mock_repo().await;
        let uuid = Uuid::now_v7();
        let result = search_person_by_id(State(repo), Path(uuid)).await;

        assert_eq!(result, Err(StatusCode::UNPROCESSABLE_ENTITY))
    }

    #[tokio::test]
    async fn response_should_be_422_in_get_by_term() {
        let State(repo) = initialize_mock_repo().await;
        let query: HashMap<String, String> = HashMap::new();
        let result = search_people_by_term(State(repo), Query(query)).await;

        assert_eq!(result, Err(StatusCode::UNPROCESSABLE_ENTITY))
    }

    async fn initialize_mock_repo() -> State<AppState> {
        let people = Arc::new(Mutex::new(HashMap::new()));
        let person = Person::new(
            "Roberto".to_string(),
            "Ro".to_string(),
            date!(2010 - 12 - 3),
            vec!["c".to_string()],
        );
        let id = person.id;
        people.lock().await.insert(id, person);
        State(people)
    }

    async fn get_first_elem_of_repo(map: &AppState) -> Result<Person, &'static str> {
        for (_, person) in map.lock().await.clone().iter() {
            return Ok(person.clone());
        }
        Err("não existe elementos")
    }
}
