use axum::{
    extract::{Path,State},
    http::StatusCode,
    Json
};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use serde_json::{json, Value};
#[derive(Serialize, Deserialize, Debug,sqlx::FromRow)]
pub struct NewTodo{
    title: String,
    priority: Option<String>
}

pub async fn add_todo(State(pool): State<PgPool>, Json(todo): Json<NewTodo>)-> Result<Json<Value>,(StatusCode,String)> {
    let resp = sqlx::query("INSERT INTO todos (title, priority) VALUES ($1, $2) RETURNING title, priority")
                                                     .bind(&todo.title)
                                                     .bind(&todo.priority)
                                                     .execute(&pool)
                                                     .await
                                                     .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR,
                                                                format!("Error is {err}")))?;
                            Ok(Json(json!(todo)))           

}

pub async fn add_todos(State(pool): State<PgPool>, Json(todos): Json<Vec<NewTodo>>)-> Result<Json<Value>,(StatusCode,String)> {
    

    for todo in todos.iter(){
        let resp = sqlx::query("INSERT INTO todos (title, priority) VALUES ($1, $2) RETURNING title, priority")
                                                     .bind(&todo.title)
                                                     .bind(&todo.priority)
                                                     .execute(&pool)
                                                     .await
                                                     .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR,
                                                                format!("Error is {err}")))?;
    }
    
    
                            Ok(Json(json!(todos)))           

}

pub async fn get_all_todos(State(pool): State<PgPool>) -> Result<Json<Vec<NewTodo>>,(StatusCode,String)> {
    let result = sqlx::query_as("SELECT * from todos")
                       .fetch_all(&pool)
                       .await
                       .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error is {err}")))?;
                        Ok(Json(result))

}

pub async fn get_one_todo(State(pool):State<PgPool>, Path(title): Path<String>) -> Result<Json<NewTodo>,(StatusCode,String)>{
    let result = sqlx::query_as("SELECT title,priority FROM todos WHERE title = $1")
                                                            .bind(title)
                                                            .fetch_one(&pool)
                                                            .await
                                                            .map_err(|err| match err {
                                                                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
                                                                       format!("Error is {err}")),
                                                                _ => (StatusCode::INTERNAL_SERVER_ERROR,format!("Error is {err}"))       

                                                            })?;
                                                            Ok(Json(result))
}

pub async fn delete_todo(State(pool):State<PgPool>, Path(title): Path<String>) -> Result<Json<Value>,(StatusCode,String)>{
    let result = sqlx::query("DELETE FROM todos WHERE title = $1")
                                                            .bind(title)
                                                            .execute(&pool)
                                                            .await
                                                            .map_err(|err| match err {
                                                                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
                                                                       format!("Error is {err}")),
                                                                _ => (StatusCode::INTERNAL_SERVER_ERROR,format!("Error is {err}"))       

                                                            })?;
                                                            Ok(Json(json!({"msg": "Todo deleted successfully!"})))
}

pub async fn update_todo(State(pool):State<PgPool>, Path(title): Path<String>, 
                                           Json(todo): Json<NewTodo>) -> Result<Json<Value>,(StatusCode,String)>{
    let result = sqlx::query("UPDATE todos set title=$1, priority=$2 WHERE title = $3")
                                                            .bind(&todo.title)
                                                            .bind(&todo.priority)
                                                            .bind(title)
                                                            .execute(&pool)
                                                            .await
                                                            .map_err(|err| match err {
                                                                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
                                                                       format!("Error is {err}")),
                                                                _ => (StatusCode::INTERNAL_SERVER_ERROR,format!("Error is {err}"))       

                                                            })?;
                                                            Ok(Json(json!({"msg": "Todo updated successfully!"})))
}