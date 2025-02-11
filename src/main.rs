use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use chrono::NaiveDateTime;

mod lib;
mod model;
mod schema;

use crate::lib::establish_connection;
use crate::model::{NewTodo, Todo};
use crate::schema::todo;

#[derive(Deserialize)]
struct GetTodo {
    userid: String,
}

#[derive(Deserialize)]
struct CreateTodo {
    user_id: i32,
    title: String,
    body: String,
    due_date: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
struct UpdateTodo {
    todoid: i32,
    title: String,
    done: bool,
}

/// Listar todas as tarefas de um usuário
async fn todo(todo: web::Query<GetTodo>) -> impl Responder {
    let connection = &mut establish_connection();
    match todo::table
        .filter(todo::user_id.eq(todo.userid.parse::<i32>().unwrap()))
        .load::<Todo>(connection)
    {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Criar uma nova tarefa
async fn todocreate(todo: web::Json<CreateTodo>) -> impl Responder {
    let connection = &mut establish_connection();

    let new_todo = NewTodo {
        user_id: todo.user_id,
        title: todo.title.clone(),
        body: todo.body.clone(),
        done: false,
        due_date: todo.due_date,
    };

    match diesel::insert_into(todo::table)
        .values(&new_todo)
        .returning(Todo::as_returning()) // Se necessário
        .get_result::<Todo>(connection)
    {
        Ok(inserted_todo) => HttpResponse::Created().json(inserted_todo),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao criar Todo"),
    }
}

/// Atualizar uma tarefa
async fn todoupdate(todo: web::Json<UpdateTodo>) -> impl Responder {
    let connection = &mut establish_connection();

    match diesel::update(todo::table.find(todo.todoid))
        .set((todo::title.eq(todo.title.clone()), todo::done.eq(todo.done)))
        .returning(Todo::as_returning())
        .get_result::<Todo>(connection)
    {
        Ok(updated_todo) => HttpResponse::Ok().json(updated_todo),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao atualizar Todo"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/todo", web::get().to(todo))
            .route("/todo", web::post().to(todocreate))
            .route("/todo", web::put().to(todoupdate))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
