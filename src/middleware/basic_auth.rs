use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use sea_orm::DatabaseConnection;
use crate::repositories::UserRepository;

pub async fn basic_auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let db = req
        .app_data::<actix_web::web::Data<DatabaseConnection>>()
        .expect("Database connection not found")
        .get_ref();

    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        Some(pwd) => {
            match UserRepository::find_by_username(db, username).await {
                Ok(Some(user)) => {
                    match bcrypt::verify(pwd.as_ref(), &user.password_hash) {
                        Ok(true) => Ok(req),
                        _ => {
                            let error = actix_web::error::ErrorUnauthorized("Invalid credentials");
                            Err((error, req))
                        }
                    }
                }
                _ => {
                    let error = actix_web::error::ErrorUnauthorized("Invalid credentials");
                    Err((error, req))
                }
            }
        }
        None => {
            let error = actix_web::error::ErrorUnauthorized("Password required");
            Err((error, req))
        }
    }
}
