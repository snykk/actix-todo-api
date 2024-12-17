use actix_web::{post, web, HttpResponse};
use crate::{repositories::auth, utils::jwt};
use crate::models::auth::{RegisterRequest, LoginRequest, LoginResponse, NewUser};
use crate::models::response::BaseResponse; // Impor BaseResponse
use bcrypt::{hash, verify};
use crate::config::db::Database;

#[post("/register")]
pub async fn register(
    database: web::Data<Database>, 
    new_user: web::Json<RegisterRequest>
) -> HttpResponse {
    let mut conn = database.get_connection();

    let password_hash = hash(&new_user.password, 10).expect("Failed to hash password");

    let user = auth::create_user(&mut conn, NewUser {
        username: new_user.username.clone(),
        password_hash,
    });

    let response = BaseResponse {
        status: true,
        message: "User registered successfully".to_string(),
        data: Some(user),
    };

    HttpResponse::Created().json(response)
}

#[post("/login")]
pub async fn login(
    database: web::Data<Database>, 
    credentials: web::Json<LoginRequest>
) -> HttpResponse {
    let mut conn = database.get_connection();

    if let Some(user) = auth::find_user_by_username(&mut conn, &credentials.username) {
        if verify(&credentials.password, &user.password_hash).unwrap() {
            let token = jwt::generate_token(user.id); // Generate token JWT

            let response = BaseResponse {
                status: true,
                message: "Login successful".to_string(),
                data: Some(LoginResponse { token }),
            };

            return HttpResponse::Ok().json(response);
        }
    }

    let response = BaseResponse::<()>{
        status: false,
        message: "Invalid credentials".to_string(),
        data: None,
    };

    HttpResponse::Unauthorized().json(response)
}
