use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<super::video_dto::VideoResponse>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePersonRequest {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100))]
    pub last_name: String,
    #[validate(range(min = 1, max = 120))]
    pub age: i32,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePersonRequest {
    #[validate(length(min = 1, max = 100))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub last_name: Option<String>,
    #[validate(range(min = 1, max = 120))]
    pub age: Option<i32>,
    #[validate(email)]
    pub email: Option<String>,
}
