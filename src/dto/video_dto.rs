use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoResponse {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub url: String,
    pub author_id: i32,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<super::person_dto::PersonResponse>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVideoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(url)]
    pub url: String,
    pub author_id: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateVideoRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub description: Option<String>,
    #[validate(url)]
    pub url: Option<String>,
    pub author_id: Option<i32>,
}
