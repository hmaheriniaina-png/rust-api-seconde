use sea_orm::DatabaseConnection;
use crate::dto::{LoginRequest, RegisterRequest, LoginResponse, RegisterResponse};
use crate::mappers::UserMapper;
use crate::repositories::UserRepository;
use crate::utils::{ApiError, generate_token};

pub struct AuthService;

impl AuthService {
    pub async fn register(
        db: &DatabaseConnection,
        req: RegisterRequest,
        jwt_secret: &str,
    ) -> Result<RegisterResponse, ApiError> {
        let existing_user = UserRepository::find_by_username(db, &req.username).await?;

        if existing_user.is_some() {
            return Err(ApiError::Conflict("Username already exists".to_string()));
        }

        let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        let user = UserRepository::create(db, req.username, password_hash).await?;

        let user_response = UserMapper::to_response(&user);

        Ok(RegisterResponse {
            message: "User registered successfully".to_string(),
            user: user_response,
        })
    }

    pub async fn login(
        db: &DatabaseConnection,
        req: LoginRequest,
        jwt_secret: &str,
    ) -> Result<LoginResponse, ApiError> {
        let user = UserRepository::find_by_username(db, &req.username)
            .await?
            .ok_or_else(|| ApiError::Unauthorized("Invalid credentials".to_string()))?;

        let password_valid = bcrypt::verify(&req.password, &user.password_hash)
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        if !password_valid {
            return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
        }

        let token = generate_token(user.id, &user.username, jwt_secret)
            .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        Ok(LoginResponse {
            message: "Login successful".to_string(),
            token,
        })
    }

    pub async fn get_profile(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<crate::dto::UserResponse, ApiError> {
        let user = UserRepository::find_by_id(db, user_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

        Ok(UserMapper::to_response(&user))
    }
}
