use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::controllers::{
    health_check, register, login, get_profile, get_all_persons, get_person, create_person,
    update_person, delete_person, get_all_videos, get_video, create_video, update_video,
    delete_video,
};
use crate::middleware::{jwt_validator, basic_auth_validator};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(register)
        .service(login)
        .service(
            web::scope("")
                .wrap(HttpAuthentication::bearer(jwt_validator))
                .service(get_profile)
                .service(create_person)
                .service(update_person)
                .service(delete_person)
                .service(get_all_videos)
                .service(get_video)
                .service(create_video)
                .service(update_video)
                .service(delete_video),
        )
        .service(
            web::scope("")
                .wrap(HttpAuthentication::basic(basic_auth_validator))
                .service(get_all_persons)
                .service(get_person),
        );
}
