use sea_orm::*;
use crate::models::video::{self, Entity as Video};

pub struct VideoRepository;

impl VideoRepository {
    pub async fn create(
        db: &DatabaseConnection,
        title: String,
        description: String,
        url: String,
        author_id: i32,
    ) -> Result<video::Model, DbErr> {
        let now = chrono::Utc::now();
        let video = video::ActiveModel {
            title: Set(title),
            description: Set(description),
            url: Set(url),
            author_id: Set(author_id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        video.insert(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<video::Model>, DbErr> {
        Video::find().all(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<video::Model>, DbErr> {
        Video::find_by_id(id).one(db).await
    }

    pub async fn find_by_author_id(
        db: &DatabaseConnection,
        author_id: i32,
    ) -> Result<Vec<video::Model>, DbErr> {
        Video::find()
            .filter(video::Column::AuthorId.eq(author_id))
            .all(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        title: Option<String>,
        description: Option<String>,
        url: Option<String>,
        author_id: Option<i32>,
    ) -> Result<video::Model, DbErr> {
        let video = Video::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Video not found".to_string()))?;

        let mut video: video::ActiveModel = video.into();

        if let Some(title) = title {
            video.title = Set(title);
        }
        if let Some(description) = description {
            video.description = Set(description);
        }
        if let Some(url) = url {
            video.url = Set(url);
        }
        if let Some(author_id) = author_id {
            video.author_id = Set(author_id);
        }

        video.updated_at = Set(chrono::Utc::now());
        video.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        let video = Video::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Video not found".to_string()))?;

        video.delete(db).await
    }
}
