// use axum::{
//      extract::{Path, State}, 
//      routing::{delete, get, post}, 
//      Json, 
//      Router
// };

// mod model;
// mod error;

// use crate::model::{ModelController,Todo,TodoBody};
// pub use self::error::{Result, Error};

// async fn create_todo(
//     mc: State<ModelController>, Json(todo_body): Json<TodoBody>
// )-> Result<Json<Todo>> {
//     let todo = mc.create_todo(todo_body).await?;

//     Ok(Json(todo))


// }

// async fn list_todos(
//     mc: State<ModelController>
// )-> Result<Json<Vec<Todo>>>{
//     let todos = mc.list_todos().await?;

//     Ok(Json(todos))

// }

// async fn delete_todo(
//     mc: State<ModelController>, Path(id): Path<u64>
// ) -> Result<Json<Todo>> {
//     let todo = mc.delete_todo(id).await?;
//     Ok(Json(todo))

// }

// fn routes(mc: ModelController)-> Router{
//     Router::new()
//          .route("/todos",get(list_todos).post(create_todo))
//          .route("/todos/:id", delete(delete_todo))
//          .with_state(mc)

// }


// #[tokio::main]
// async fn main() -> Result<()>{

//     let mc = ModelController::new().await?;

//     let app = Router::new().nest("/",routes(mc.clone()));


//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     println!("App runnning on {:#?}",listener.local_addr());
//     axum::serve(listener,app).await.unwrap();

//     Ok(())




//2nd code

// #![allow(unused)]
// use axum::{
//     body::Body,
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     routing::{get, post},
//     Json, Router,
// };
// use serde::{Deserialize, Serialize};
// use sqlx::{sqlite::SqlitePoolOptions, PgPool};
// use sqlx::postgres::{PgPoolOptions, PgRow};
// use sqlx::{FromRow, Row};

// #[derive(Serialize,Deserialize)]
// struct Todo {
//     title: String
// }

// // Handler for /create-user
// async fn create_user(Json(todo): Json<Todo>) -> Json<Todo> {
    
//     Json(todo)
// }
// // Handler for /users
// async fn list_todos() -> Json<Vec<Todo>> {
//     let todos = vec![
//         Todo {
//             title: "C++ code".to_string(),
//         },
//         Todo {
//             title: "Java code".to_string()
//         },
//     ];
//     Json(todos)
// }

// #[tokio::main]
// async fn main()  -> Result<(), sqlx::Error>{
    
//     dotenv::dotenv().ok();
//     let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

//     let pool = sqlx::PgPool::connect(&database_url)
//             .await
//             .expect("Error with pool connection");

//         sqlx::query!(
//             r#"
//             CREATE TABLE IF NOT EXISTS Todo (
//                 title TEXT NOT NULL
//             )
//             "#
//         )
//         .execute(&pool)
//         .await?;


//     // Define Routes
//     let app = Router::new()
//         .route("/", get(|| async { "Hello, Rust!" }))
//         .route("/add-todo", post(create_user))
//         .route("/todos", get(list_todos))
//         .with_state(pool);

//     println!("Running on http://localhost:3000");
    

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
//     axum::serve(listener,app).await.unwrap();

//     Ok(())
// }



use axum::{
    body::Body, extract::{Path, State}, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post, put}, Json, Router
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::postgres::PgPool;


mod routes;
use routes::run;
mod handlers;
// pub use self::error::{Error,Result};




// #[derive(Serialize, Deserialize, Debug,sqlx::FromRow)]
// pub struct Todo {
//     title: String,
//     priority: Option<String>,
// }

// #[derive(Serialize, Deserialize, sqlx::FromRow)]
// pub struct Users {
//     name: String,
//     email: Option<String>,
// }

// async fn create_user(Json(user): Json<User>, pool: &PgPool) -> Result<Json<User>> {
//     let inserted_user = sqlx::query!(
//         "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
//         user.name,
//         user.email
//     )
//     .fetch_one(pool)
//     .await?;

//     Ok(Json(User {
//         id: inserted_user.id,
//         name: inserted_user.name,
//         email: inserted_user.email,
//     }))
// }

// // Handler for /users
// async fn list_users(pool: &PgPool) -> Result<Json<Vec<User>>> {
//     let users = sqlx::query_as!(
//         User,
//         r#"SELECT id, name, email FROM users"#
//     )
//     .fetch_all(pool)
//     .await?;

//     Ok(Json(users))
// } 

#[tokio::main]
async fn main()  {
    // Create a database connection pool
    // dotenv::dotenv().ok();
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // let pool = PgPool::connect(&database_url)
    //         .await
    //         .expect("Error with pool connection");


    // sqlx::query!(
    //     r#"
    //     CREATE TABLE todos (
    //         title TEXT NOT NULL,
    //         priority TEXT
    //     )
    //     "#
    // )
    // .execute(&pool)
    // .await;
  


    run().await
    // Define Routes
    // let app = Router::new()
    //     .route("/", get(|| async { "Hello, Rust!" }))
    //     .route("/add-todos", post(handlers::add_todo))
    //     .route("/todos", get(handlers::get_all_todos))
    //     .route("/todo/:title", get(handlers::get_one_todo))
    //     .route("/delete/:title", delete(handlers::delete_todo))
    //     .route("/update/:title", put(handlers::update_todo))
    //     .with_state(pool);

    // println!("Running on http://localhost:3000");

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    // axum::serve(listener,app).await.unwrap();

    //Ok(())
}

// pub async fn add_todo(State(pool): State<PgPool>, Json(todo): Json<Todo>)-> Result<Json<Value>,(StatusCode,String)> {
//     let resp = sqlx::query("INSERT INTO todos (title, priority) VALUES ($1, $2) RETURNING title, priority")
//                                                      .bind(&todo.title)
//                                                      .bind(&todo.priority)
//                                                      .execute(&pool)
//                                                      .await
//                                                      .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR,
//                                                                 format!("Error is {err}")))?;
//                             Ok(Json(json!(todo)))           

// }

// pub async fn get_all_todos(State(pool): State<PgPool>) -> Result<Json<Vec<Todo>>,(StatusCode,String)> {
//     let result = sqlx::query_as("SELECT * from todos")
//                        .fetch_all(&pool)
//                        .await
//                        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR,
//                         format!("Error is {err}")))?;
//                         Ok(Json(result))

// }

// pub async fn get_one_todo(State(pool):State<PgPool>, Path(title): Path<String>) -> Result<Json<Todo>,(StatusCode,String)>{
//     let result = sqlx::query_as("SELECT title,priority FROM todos WHERE title = $1")
//                                                             .bind(title)
//                                                             .fetch_one(&pool)
//                                                             .await
//                                                             .map_err(|err| match err {
//                                                                 sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
//                                                                        format!("Error is {err}")),
//                                                                 _ => (StatusCode::INTERNAL_SERVER_ERROR,format!("Error is {err}"))       

//                                                             })?;
//                                                             Ok(Json(result))
// }

// pub async fn delete_todo(State(pool):State<PgPool>, Path(title): Path<String>) -> Result<Json<Value>,(StatusCode,String)>{
//     let result = sqlx::query("DELETE FROM todos WHERE title = $1")
//                                                             .bind(title)
//                                                             .execute(&pool)
//                                                             .await
//                                                             .map_err(|err| match err {
//                                                                 sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
//                                                                        format!("Error is {err}")),
//                                                                 _ => (StatusCode::INTERNAL_SERVER_ERROR,format!("Error is {err}"))       

//                                                             })?;
//                                                             Ok(Json(json!({"msg": "Todo deleted successfully!"})))
// }

// pub async fn update_todo(State(pool):State<PgPool>, Path(title): Path<String>, 
//                                            Json(todo): Json<Todo>) -> Result<Json<Value>,(StatusCode,String)>{
//     let result = sqlx::query("UPDATE todos set title=$1, priority=$2 WHERE title = $3")
//                                                             .bind(&todo.title)
//                                                             .bind(&todo.priority)
//                                                             .bind(title)
//                                                             .execute(&pool)
//                                                             .await
//                                                             .map_err(|err| match err {
//                                                                 sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
//                                                                        format!("Error is {err}")),
//                                                                 _ => (StatusCode::INTERNAL_SERVER_ERROR,format!("Error is {err}"))       

//                                                             })?;
//                                                             Ok(Json(json!({"msg": "Todo updated successfully!"})))
// }