use axum::Json;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::person::Person;
pub struct Repo {
    pool: PgPool,
}

impl Repo {
    pub async fn new(database_url: String) -> Repo {
        Repo {
            pool: PgPool::connect(&database_url)
                .await
                .expect("cannot connect to database"),
        }
    }

    pub async fn select_by_id(&self, id: Uuid) -> Result<Person, sqlx::Error> {
        let row = sqlx::query_as(
            "SELECT id, name, nick, birth_date, stack 
             FROM people
             WHERE id = ($1)",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn select_by_term(&self, query: String) -> Result<Vec<Person>, sqlx::Error> {
        let row = sqlx::query_as(
            "SELECT id, name, nick, birth_date, stack
             FROM people
             WHERE search ~ $1
             LIMIT 50",
        )
        .bind(query)
        .fetch_all(&self.pool)
        .await?;

        Ok(row)
    }
    pub async fn count_amount_of_person(&self) -> Result<i64, sqlx::Error> {
        let row: DataLength = sqlx::query_as("SELECT count(*) FROM people")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.count.unwrap())
    }
    pub async fn create_new_person(
        &self,
        person: Json<Person>,
    ) -> Result<Json<Person>, sqlx::Error> {
        sqlx::query(
            "INSERT INTO people (id, name, nick, birth_date, stack) VALUES
            ($1, $2, $3, $4, $5)",
        )
        .bind(&person.id)
        .bind(&person.name)
        .bind(&person.nick)
        .bind(&person.birth_date)
        .bind(&person.stack)
        .execute(&self.pool)
        .await?;

        Ok(person)
    }
}

#[derive(FromRow)]
struct DataLength {
    count: Option<i64>,
}
