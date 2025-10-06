use sea_orm::DatabaseConnection;
use crate::dto::{CreateVideoRequest, UpdateVideoRequest, VideoResponse};
use crate::mappers::{VideoMapper, PersonMapper};
use crate::repositories::{VideoRepository, PersonRepository};
use crate::utils::ApiError;

pub struct VideoService;

impl VideoService {
    pub async fn create(
        db: &DatabaseConnection,
        req: CreateVideoRequest,
    ) -> Result<VideoResponse, ApiError> {
        let author = PersonRepository::find_by_id(db, req.author_id)
            .await?
            .ok_or_else(|| ApiError::BadRequest("Author not found".to_string()))?;

        let video = VideoRepository::create(
            db,
            req.title,
            req.description,
            req.url,
            req.author_id,
        )
        .await?;

        let author_response = PersonMapper::to_response(&author);
        Ok(VideoMapper::to_response_with_author(&video, author_response))
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<VideoResponse>, ApiError> {
        let videos = VideoRepository::find_all(db).await?;
        let mut responses = Vec::new();

        for video in videos {
            if let Some(author) = PersonRepository::find_by_id(db, video.author_id).await? {
                let author_response = PersonMapper::to_response(&author);
                responses.push(VideoMapper::to_response_with_author(&video, author_response));
            } else {
                responses.push(VideoMapper::to_response(&video));
            }
        }

        Ok(responses)
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<VideoResponse, ApiError> {
        let video = VideoRepository::find_by_id(db, id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Video not found".to_string()))?;

        let author = PersonRepository::find_by_id(db, video.author_id).await?;

        if let Some(author) = author {
            let author_response = PersonMapper::to_response(&author);
            Ok(VideoMapper::to_response_with_author(&video, author_response))
        } else {
            Ok(VideoMapper::to_response(&video))
        }
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        req: UpdateVideoRequest,
    ) -> Result<VideoResponse, ApiError> {
        if let Some(author_id) = req.author_id {
            PersonRepository::find_by_id(db, author_id)
                .await?
                .ok_or_else(|| ApiError::BadRequest("Author not found".to_string()))?;
        }

        let video = VideoRepository::update(
            db,
            id,
            req.title,
            req.description,
            req.url,
            req.author_id,
        )
        .await?;

        let author = PersonRepository::find_by_id(db, video.author_id).await?;

        if let Some(author) = author {
            let author_response = PersonMapper::to_response(&author);
            Ok(VideoMapper::to_response_with_author(&video, author_response))
        } else {
            Ok(VideoMapper::to_response(&video))
        }
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), ApiError> {
        VideoRepository::delete(db, id).await?;
        Ok(())
    }
}
