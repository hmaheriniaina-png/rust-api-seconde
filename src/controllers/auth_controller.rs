use actix_web::{post, get, web, HttpResponse, HttpRequest};
use sea_orm::DatabaseConnection;
use validator::Validate;
use crate::dto::{LoginRequest, RegisterRequest};
use crate::services::AuthService;
use crate::utils::{ApiError, Claims};

#[post("/api/v1/auth/register")]
pub async fn register(
    db: web::Data<DatabaseConnection>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string());
    let response = AuthService::register(db.get_ref(), req.into_inner(), &jwt_secret).await?;

    Ok(HttpResponse::Created().json(response))
}

#[post("/api/v1/auth/login")]
pub async fn login(
    db: web::Data<DatabaseConnection>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default-secret".to_string());
    let response = AuthService::login(db.get_ref(), req.into_inner(), &jwt_secret).await?;

    Ok(HttpResponse::Ok().json(response))
}

#[get("/api/v1/profile")]
pub async fn get_profile(
    db: web::Data<DatabaseConnection>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| ApiError::Unauthorized("Unauthorized".to_string()))?;

    let user = AuthService::get_profile(db.get_ref(), claims.sub).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": user
    })))
}
