use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{persistence::Repo, person::Person};

type AppState = Arc<Mutex<Repo>>;

pub async fn search_people_by_term(
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<Vec<Person>>), StatusCode> {
    let term = match query.get("t") {
        Some(term) => term,
        None => return Err(StatusCode::UNPROCESSABLE_ENTITY),
    };
    let result = state.lock().await.select_by_term(term.to_string()).await;

    let people = match result {
        Ok(people) => {
            if people.is_empty() {
                return Err(StatusCode::UNPROCESSABLE_ENTITY);
            }
            people
        }
        Err(_) => return Err(StatusCode::UNPROCESSABLE_ENTITY),
    };

    Ok((StatusCode::OK, Json(people)))
}

pub async fn search_person_by_id(
    State(state): State<AppState>,
    Path(person_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Person>), StatusCode> {
    let result = state.lock().await.select_by_id(person_id).await;

    match result {
        Ok(person) => Ok((StatusCode::OK, Json(person))),
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
pub async fn create_person(
    State(state): State<AppState>,
    person: Json<Person>,
) -> Result<(StatusCode, Json<Person>), StatusCode> {
    let result = state.lock().await.create_new_person(person).await;

    match result {
        Ok(person) => Ok((StatusCode::CREATED, person)),
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}
pub async fn amount_of_people(
    State(state): State<AppState>,
) -> Result<(StatusCode, String), StatusCode> {
    let result = state.lock().await.count_amount_of_person().await;
    match result {
        Ok(count) => Ok((StatusCode::OK, count.to_string())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// #[cfg(test)]
// mod endpoints_tests {
//     use axum::extract::{Path, State};
//     use std::sync::Arc;
//     use time::macros::date;

//     use super::*;
//     use crate::{person::Person, routes::search_person_by_id};

//     #[tokio::test]
//     async fn search_people_by_term_contain_term() {
//         let repo = initialize_mock_repo().await; // init with person Roberto
//         let mut query: HashMap<String, String> = HashMap::new();
//         query.insert("t".to_string(), "berto".to_string());
//         let (_, people_as_json) = search_people_by_term(repo, Query(query)).await.unwrap();

//         assert!(people_as_json.contains("berto"))
//     }

//     #[tokio::test]
//     async fn search_person_by_id_test() {
//         let State(repo) = initialize_mock_repo().await;
//         let first_person = get_first_elem_of_repo(&repo).await.unwrap();
//         let (_, person_from_fn) = search_person_by_id(State(repo), Path(first_person.id))
//             .await
//             .unwrap();

//         let person_from_fn: Person = Person::deserialize_from_string(person_from_fn).unwrap();
//         assert_eq!(first_person, person_from_fn);
//     }
//     #[tokio::test]
//     async fn create_valid_person() {
//         let repo: AppState = Arc::new(Mutex::new(HashMap::new()));
//         let person = Person::new(
//             "John".to_string(),
//             "Doe".to_string(),
//             date!(1999 - 9 - 9),
//             Some(vec!["c".to_string(), "c++".to_string()]),
//         );
//         let person_as_json = json!(person).to_string();
//         let _ = create_person(State(repo.clone()), person_as_json).await;

//         assert_eq!(repo.lock().await.len(), 1);
//     }

//     #[tokio::test]
//     async fn create_invalid_person() {
//         let repo: AppState = Arc::new(Mutex::new(HashMap::new()));
//         let person = Person::new(
//             "John".to_string(),
//             "name with more than 30 characters is not permitted".to_string(),
//             date!(1999 - 9 - 9),
//             Some(vec!["c".to_string(), "c++".to_string()]),
//         );

//         let result = create_person(State(repo), json!(person).to_string()).await;

//         assert_eq!(result, Err(StatusCode::UNPROCESSABLE_ENTITY))
//     }

//     #[tokio::test]
//     async fn count_amount_of_person() {
//         let repo: AppState = Arc::new(Mutex::new(HashMap::new()));
//         let person = Person::new(
//             "John".to_string(),
//             "name with more than 30 characters is not permitted".to_string(),
//             date!(1999 - 9 - 9),
//             Some(vec!["c".to_string(), "c++".to_string()]),
//         );
//         repo.lock().await.insert(person.id, person);

//         let (_, amount_people) = amount_of_people(State(repo)).await;

//         assert_eq!(1, amount_people.parse::<usize>().unwrap())
//     }

//     #[tokio::test]
//     async fn response_should_be_422_in_get_by_id() {
//         let State(repo) = initialize_mock_repo().await;
//         let uuid = Uuid::now_v7();
//         let result = search_person_by_id(State(repo), Path(uuid)).await;

//         assert_eq!(result, Err(StatusCode::UNPROCESSABLE_ENTITY))
//     }

//     #[tokio::test]
//     async fn response_should_be_422_in_get_by_term() {
//         let State(repo) = initialize_mock_repo().await;
//         let query: HashMap<String, String> = HashMap::new();
//         let result = search_people_by_term(State(repo), Query(query)).await;

//         assert_eq!(result, Err(StatusCode::UNPROCESSABLE_ENTITY))
//     }

//     async fn initialize_mock_repo() -> State<AppState> {
//         let people = Arc::new(Mutex::new(HashMap::new()));
//         let person = Person::new(
//             "Roberto".to_string(),
//             "Ro".to_string(),
//             date!(2010 - 12 - 3),
//             Some(vec!["c".to_string()]),
//         );
//         let id = person.id;
//         people.lock().await.insert(id, person);
//         State(people)
//     }

//     async fn get_first_elem_of_repo(map: &AppState) -> Result<Person, &'static str> {
//         for (_, person) in map.lock().await.clone().iter() {
//             return Ok(person.clone());
//         }
//         Err("não existe elementos")
//     }
// }
