use actix_web::{get, post, put, delete, web, HttpResponse};
use sea_orm::DatabaseConnection;
use validator::Validate;
use crate::dto::{CreatePersonRequest, UpdatePersonRequest};
use crate::services::PersonService;
use crate::utils::ApiError;

#[get("/api/v1/persons")]
pub async fn get_all_persons(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, ApiError> {
    let persons = PersonService::get_all(db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(persons))
}

#[get("/api/v1/persons/{id}")]
pub async fn get_person(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let person = PersonService::get_by_id(db.get_ref(), id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(person))
}

#[post("/api/v1/persons")]
pub async fn create_person(
    db: web::Data<DatabaseConnection>,
    req: web::Json<CreatePersonRequest>,
) -> Result<HttpResponse, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let person = PersonService::create(db.get_ref(), req.into_inner()).await?;
    Ok(HttpResponse::Created().json(person))
}

#[put("/api/v1/persons/{id}")]
pub async fn update_person(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    req: web::Json<UpdatePersonRequest>,
) -> Result<HttpResponse, ApiError> {
    req.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let person = PersonService::update(db.get_ref(), id.into_inner(), req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(person))
}

#[delete("/api/v1/persons/{id}")]
pub async fn delete_person(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    PersonService::delete(db.get_ref(), id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Person deleted successfully"
    })))
}
