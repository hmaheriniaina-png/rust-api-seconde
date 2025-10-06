use crate::dto::PersonResponse;
use crate::models::person;

pub struct PersonMapper;

impl PersonMapper {
    pub fn to_response(model: &person::Model) -> PersonResponse {
        PersonResponse {
            id: model.id,
            first_name: model.first_name.clone(),
            last_name: model.last_name.clone(),
            age: model.age,
            email: model.email.clone(),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
            videos: None,
        }
    }

    pub fn to_response_with_videos(
        model: &person::Model,
        videos: Vec<crate::dto::VideoResponse>,
    ) -> PersonResponse {
        PersonResponse {
            id: model.id,
            first_name: model.first_name.clone(),
            last_name: model.last_name.clone(),
            age: model.age,
            email: model.email.clone(),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
            videos: Some(videos),
        }
    }
}
