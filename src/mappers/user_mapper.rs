use crate::dto::UserResponse;
use crate::models::user;

pub struct UserMapper;

impl UserMapper {
    pub fn to_response(model: &user::Model) -> UserResponse {
        UserResponse {
            id: model.id,
            username: model.username.clone(),
            created_at: model.created_at.to_rfc3339(),
            updated_at: model.updated_at.to_rfc3339(),
        }
    }
}
