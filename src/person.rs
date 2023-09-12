#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Person {
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "apelido")]
    pub nick: String,
    #[serde(rename = "nascimento", with = "date_format")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}

impl Person {
    pub fn new(name: String, nick: String, birth_date: Date, stack: Option<Vec<String>>) -> Person {
        let person = Person {
            id: Uuid::now_v7(),
            name,
            nick,
            birth_date,
            stack,
        };

        match Self::apply_rules(&person) {
            Ok(_) => person,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn deserialize_from_string(string: String) -> Result<Person, PersonParserError> {
        let person_result: Result<Person, serde_json::Error> = serde_json::from_str(&string);

        Self::apply_rules(&person_result.unwrap())
    }

    fn apply_rules(person: &Person) -> Result<Person, PersonParserError> {
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

        Ok(person.clone())
    }
}

#[derive(Debug)]
pub enum PersonParserError {
    JsonError(serde_json::Error),
    LengthError(&'static str),
}

#[cfg(test)]
mod person_tests {

    use time::macros::date;

    use super::Person;

    #[test]
    fn deserialize_without_id() {
        let json_str = "{
            \"nome\": \"John Doe\",
            \"apelido\": \"JD\",
            \"nascimento\": \"1999-09-19\",
            \"stack\": [\"Rust\", \"Python\", \"JavaScript\"]
          }
          "
        .to_string();

        let person_deserialized = Person::deserialize_from_string(json_str).unwrap();
        assert_eq!(person_deserialized.id.to_string().len(), 36);
    }

    #[test]
    #[should_panic]
    fn create_person_with_invalid_nickname() {
        let _ = Person::new(
            "name".to_string(),
            "nickname must be smaller than 30 characters".to_string(),
            date!(1999 - 09 - 19),
            Some(vec!["".to_string()]),
        );
    }

    #[test]
    #[should_panic]
    fn create_person_with_invalid_name() {
        let _ = Person::new(
            "name with more than 100 characters is not allowed, ------------------------------------------------------------------------------------------------------------------------------".to_string(),
            "nickname".to_string(),
            date!(1999 - 09 - 19),
            Some(vec!["".to_string()]),
        );
    }

    #[test]
    #[should_panic]
    fn create_person_with_invalid_date_json() {
        let json_str = "{
            \"nome\": \"John Doe\",
            \"apelido\": \"JD\",
            \"nascimento\": \"1999-13-19\",
            \"stack\": [\"Rust\", \"Python\", \"JavaScript\"]
          }
          "
        .to_string();

        Person::deserialize_from_string(json_str).unwrap();
    }

    #[test]
    #[should_panic]
    fn create_person_with_invalid_stack() {
        let _ = Person::new(
            "name".to_string(),
            "nickname".to_string(),
            date!(1999 - 09 - 19),
            Some(vec![
                "stack should contain items with less than 30 characters".to_string(),
            ]),
        );
    }
}
