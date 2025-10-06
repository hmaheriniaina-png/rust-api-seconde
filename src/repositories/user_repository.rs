use sea_orm::*;
use crate::models::user::{self, Entity as User};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        db: &DatabaseConnection,
        username: String,
        password_hash: String,
    ) -> Result<user::Model, DbErr> {
        let now = chrono::Utc::now();
        let user = user::ActiveModel {
            username: Set(username),
            password_hash: Set(password_hash),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        user.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        username: Option<String>,
    ) -> Result<user::Model, DbErr> {
        let user = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        let mut user: user::ActiveModel = user.into();

        if let Some(username) = username {
            user.username = Set(username);
        }

        user.updated_at = Set(chrono::Utc::now());
        user.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        let user = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

        user.delete(db).await
    }
}
