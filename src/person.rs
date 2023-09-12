#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Person {
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}

impl Person {
    pub fn new(name: String, nick: String, birth_date: Date, stack: Option<Vec<String>>) -> Person {
        Person {
            id: Uuid::now_v7(),
            name,
            nick,
            birth_date,
            stack,
        }
    }
    pub fn deserialize_from_string(string: String) -> Result<Person, PersonParserError> {
        let person_result: Result<Person, serde_json::Error> = serde_json::from_str(&string);

        Self::apply_rules(person_result)
    }

    fn apply_rules(person: Result<Person, serde_json::Error>) -> Result<Person, PersonParserError> {
        match person {
            Ok(person) => {
                if &person.nick.len() > &32 {
                    return Err(PersonParserError::LengthError(
                        "nick length must be smaller than 32 characters",
                    ));
                }
                if &person.name.len() > &100 {
                    return Err(PersonParserError::LengthError(
                        "name length must be smaller than 100 characters",
                    ));
                }
                if &person.stack.clone().unwrap_or(vec!["".to_string()]).len() > &32 {
                    return Err(PersonParserError::LengthError(
                        "stack length must be smaller than 32 characters",
                    ));
                }

                for tech in person
                    .stack
                    .clone()
                    .unwrap_or(vec!["".to_string()])
                    .into_iter()
                {
                    if tech.len() > 32 {
                        return Err(PersonParserError::LengthError(
                            "Tech length must be smaller than 32 characters",
                        ));
                    }
                }
                Ok(person)
            }
            Err(e) => Err(PersonParserError::JsonError(e)),
        }
    }
}

#[derive(Debug)]
pub enum PersonParserError {
    JsonError(serde_json::Error),
    LengthError(&'static str),
}

#[cfg(test)]
mod person_tests {
    use serde_json::json;

    use super::Person;

    #[test]
    fn deserialize_without_id() {
        let json_str = "{
            \"name\": \"John Doe\",
            \"nick\": \"JD\",
            \"birth_date\": [1990,345],
            \"stack\": [\"Rust\", \"Python\", \"JavaScript\"]
          }
          "
        .to_string();

        let person_deserialized = Person::deserialize_from_string(json_str).unwrap();
        let person_serialized = json!(person_deserialized).to_string();
        assert!(person_serialized.contains("id"));
    }
}
