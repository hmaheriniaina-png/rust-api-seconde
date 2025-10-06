use actix_web::{get, post, put, delete, web, HttpResponse};
use sea_orm::DatabaseConnection;
use validator::Validate;
use crate::dto::{CreateVideoRequest, UpdateVideoRequest};
use crate::services::VideoService;
use crate::utils::ApiError;

#[get("/api/v1/videos")]
pub async fn get_all_videos(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, ApiError> {
    let videos = VideoService::get_all(db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(videos))
}

#[get("/api/v1/videos/{id}")]
pub async fn get_video(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let video = VideoService::get_by_id(db.get_ref(), id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(video))
}

#[post("/api/v1/videos")]
pub async fn create_video(
    db: web::Data<DatabaseConnection>,
    req: web::Json<CreateVideoRequest>,
) -> Result<HttpResponse, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let video = VideoService::create(db.get_ref(), req.into_inner()).await?;
    Ok(HttpResponse::Created().json(video))
}

#[put("/api/v1/videos/{id}")]
pub async fn update_video(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    req: web::Json<UpdateVideoRequest>,
) -> Result<HttpResponse, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let video = VideoService::update(db.get_ref(), id.into_inner(), req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(video))
}

#[delete("/api/v1/videos/{id}")]
pub async fn delete_video(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    VideoService::delete(db.get_ref(), id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Video deleted successfully"
    })))
}
