use sea_orm::DatabaseConnection;
use crate::dto::{CreatePersonRequest, UpdatePersonRequest, PersonResponse};
use crate::mappers::PersonMapper;
use crate::repositories::{PersonRepository, VideoRepository};
use crate::utils::ApiError;
use crate::mappers::VideoMapper;

pub struct PersonService;

impl PersonService {
    pub async fn create(
        db: &DatabaseConnection,
        req: CreatePersonRequest,
    ) -> Result<PersonResponse, ApiError> {
        let existing_person = PersonRepository::find_by_email(db, &req.email).await?;

        if existing_person.is_some() {
            return Err(ApiError::Conflict("Email already exists".to_string()));
        }

        let person = PersonRepository::create(
            db,
            req.first_name,
            req.last_name,
            req.age,
            req.email,
        )
        .await?;

        Ok(PersonMapper::to_response(&person))
    }

    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<PersonResponse>, ApiError> {
        let persons = PersonRepository::find_all(db).await?;
        let responses = persons.iter().map(|p| PersonMapper::to_response(p)).collect();
        Ok(responses)
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<PersonResponse, ApiError> {
        let person = PersonRepository::find_by_id(db, id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Person not found".to_string()))?;

        let videos = VideoRepository::find_by_author_id(db, person.id).await?;
        let video_responses: Vec<_> = videos.iter().map(|v| VideoMapper::to_response(v)).collect();

        Ok(PersonMapper::to_response_with_videos(&person, video_responses))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        req: UpdatePersonRequest,
    ) -> Result<PersonResponse, ApiError> {
        if let Some(ref email) = req.email {
            let existing_person = PersonRepository::find_by_email(db, email).await?;
            if let Some(existing) = existing_person {
                if existing.id != id {
                    return Err(ApiError::Conflict("Email already exists".to_string()));
                }
            }
        }

        let person = PersonRepository::update(
            db,
            id,
            req.first_name,
            req.last_name,
            req.age,
            req.email,
        )
        .await?;

        Ok(PersonMapper::to_response(&person))
    }

    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<(), ApiError> {
        PersonRepository::delete(db, id).await?;
        Ok(())
    }
}
