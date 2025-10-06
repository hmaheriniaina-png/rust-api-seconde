use sea_orm::*;
use crate::models::person::{self, Entity as Person};

pub struct PersonRepository;

impl PersonRepository {
    pub async fn create(
        db: &DatabaseConnection,
        first_name: String,
        last_name: String,
        age: i32,
        email: String,
    ) -> Result<person::Model, DbErr> {
        let now = chrono::Utc::now();
        let person = person::ActiveModel {
            first_name: Set(first_name),
            last_name: Set(last_name),
            age: Set(age),
            email: Set(email),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        person.insert(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<person::Model>, DbErr> {
        Person::find().all(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<person::Model>, DbErr> {
        Person::find_by_id(id).one(db).await
    }

    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<person::Model>, DbErr> {
        Person::find()
            .filter(person::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
        age: Option<i32>,
        email: Option<String>,
    ) -> Result<person::Model, DbErr> {
        let person = Person::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Person not found".to_string()))?;

        let mut person: person::ActiveModel = person.into();

        if let Some(first_name) = first_name {
            person.first_name = Set(first_name);
        }
        if let Some(last_name) = last_name {
            person.last_name = Set(last_name);
        }
        if let Some(age) = age {
            person.age = Set(age);
        }
        if let Some(email) = email {
            person.email = Set(email);
        }

        person.updated_at = Set(chrono::Utc::now());
        person.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        let person = Person::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Person not found".to_string()))?;

        person.delete(db).await
    }
}
