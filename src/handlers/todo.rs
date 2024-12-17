use actix_web::{get, post, put, delete, web, HttpResponse};
use crate::repositories::todo;
use crate::models::todo::{NewTodo, UpdateTodo};
use crate::utils::jwt::Claims;
use crate::config::db::Database;
use crate::models::response::BaseResponse;
use uuid::Uuid;

#[get("")]
pub async fn get_todos(claims: Claims, database: web::Data<Database>) -> HttpResponse {
    let mut conn = database.get_connection();

    let todos = todo::get_todos(&mut conn, claims.user_id);

    let response = BaseResponse {
        status: true,
        message: "Todos retrieved successfully".to_string(),
        data: Some(todos),
    };

    HttpResponse::Ok().json(response)
}

#[post("")]
pub async fn create_todo(
    claims: Claims, 
    new_todo: web::Json<NewTodo>, 
    database: web::Data<Database>
) -> HttpResponse {
    let mut conn = database.get_connection();

    let todo = todo::create_todo(&mut conn, NewTodo {
        user_id: claims.user_id, // Set user_id sesuai klaim JWT
        ..new_todo.into_inner() // Ambil data dari body request
    });

    let response = BaseResponse {
        status: true,
        message: "Todo created successfully".to_string(),
        data: Some(todo),
    };

    HttpResponse::Created().json(response)
}

#[put("/{id}")]
pub async fn update_todo(
    claims: Claims,
    todo_id: web::Path<Uuid>,
    changes: web::Json<UpdateTodo>,
    database: web::Data<Database>
) -> HttpResponse {
    let mut conn = database.get_connection();

    match todo::update_todo(&mut conn, todo_id.into_inner(), claims.user_id, changes.into_inner()) {
        Some(todo) => {
            let response = BaseResponse {
                status: true,
                message: "Todo updated successfully".to_string(),
                data: Some(todo),
            };
            HttpResponse::Ok().json(response)
        },
        None => {
            let response = BaseResponse::<()>{
                status: false,
                message: "Todo not found or not authorized".to_string(),
                data: None,
            };
            HttpResponse::NotFound().json(response)
        }
    }
}

#[delete("/{id}")]
pub async fn delete_todo(
    claims: Claims, 
    todo_id: web::Path<Uuid>,
    database: web::Data<Database>
) -> HttpResponse {
    let mut conn = database.get_connection();

    if todo::delete_todo(&mut conn, todo_id.into_inner(), claims.user_id) {
        HttpResponse::NoContent().finish()
    } else {
        let response = BaseResponse::<()>{
            status: false,
            message: "Todo not found or not authorized".to_string(),
            data: None,
        };
        HttpResponse::NotFound().json(response)
    }
}
