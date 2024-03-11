



use crate:: handlers;

use axum::{routing::{get,post,put,delete}, Router};
//use handlers::{add_todo,update_todo,delete_todo,get_all_todos,get_one_todo};
use sqlx::PgPool;
use sqlx::sqlite::SqlitePool;
pub async fn run(){

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPool::connect(&database_url)
            .await
            .expect("Error with pool connection");


    sqlx::query!(
        r#"
        CREATE TABLE todos (
            title TEXT NOT NULL,
            priority TEXT
        )
        "#
    )
    .execute(&pool)
    .await;
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/add-todos", post(handlers::add_todos))
        .route("/todos", get(handlers::get_all_todos))
        .route("/todo/:title", get(handlers::get_one_todo))
        .route("/delete/:title", delete(handlers::delete_todo))
        .route("/update/:title", put(handlers::update_todo))
        .with_state(pool);

    println!("Running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    axum::serve(listener,app).await.unwrap();
}