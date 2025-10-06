use crate::dto::VideoResponse;
use crate::models::video;

pub struct VideoMapper;

impl VideoMapper {
    pub fn to_response(model: &video::Model) -> VideoResponse {
        VideoResponse {
            id: model.id,
            title: model.title.clone(),
            description: model.description.clone(),
            url: model.url.clone(),
            author_id: model.author_id,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
            author: None,
        }
    }

    pub fn to_response_with_author(
        model: &video::Model,
        author: crate::dto::PersonResponse,
    ) -> VideoResponse {
        VideoResponse {
            id: model.id,
            title: model.title.clone(),
            description: model.description.clone(),
            url: model.url.clone(),
            author_id: model.author_id,
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
            author: Some(Box::new(author)),
        }
    }
}
