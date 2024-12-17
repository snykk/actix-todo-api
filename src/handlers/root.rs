use actix_web::{get, HttpResponse};
use crate::models::response::BaseResponse;

#[get("/")]
pub async fn health_check() -> HttpResponse {
    let response = BaseResponse::<()>{ 
        status: true,
        message: "Server is running".to_string(),
        data: None, 
    };

    HttpResponse::Ok().json(response)
}
