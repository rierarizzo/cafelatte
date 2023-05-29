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
    let users = user_service.get_users().await.unwrap();

    let mut response: Vec<UserResponse> = vec![];
    for user in users {
        response.push(UserResponse {
            name: user.name,
            surname: user.surname,
        });
    }

    HttpResponse::Ok().json(response)
}

#[get("/users/{id}")]
pub async fn get_user_by_id(user_service: UserService) -> impl Responder {
    if let Ok(user) = user_service.get_user_by_id(1).await {
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
