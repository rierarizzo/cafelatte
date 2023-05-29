use crate::core::ports::user_port::IUserService;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;
use std::sync::Arc;

type UserService = web::Data<Arc<dyn IUserService>>;

#[derive(Serialize)]
struct UserResponse {
    name: String,
    surname: String,
}

#[get("/users")]
pub async fn get_users(user_service: UserService) -> impl Responder {
    match user_service.get_users().await {
        Ok(users) => {
            let mut response: Vec<UserResponse> = vec![];
            for user in users {
                response.push(UserResponse {
                    name: user.name,
                    surname: user.surname,
                });
            }

            return HttpResponse::Ok().json(response);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Error"),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(user_service: UserService, path: web::Path<u8>) -> impl Responder {
    let user_id = path.into_inner();

    if let Ok(user) = user_service.get_user_by_id(user_id).await {
        HttpResponse::Ok().json(UserResponse {
            name: user.name,
            surname: user.surname,
        })
    } else {
        HttpResponse::InternalServerError().body("Error")
    }
}

#[post("/users")]
pub async fn create_user() -> impl Responder {
    HttpResponse::Created().body("create-user")
}
