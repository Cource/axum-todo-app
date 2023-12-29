use sqlx::SqlitePool;
use axum::{
    routing::{get, post, patch, delete},
    extract::{State, Path},
    Router,
    Json,
};
use serde::{Serialize,Deserialize};
use serde_json::{Value, json};

#[derive(sqlx::FromRow,Serialize, Deserialize)]
struct Todo{
    id: i8,
    name: String,
    is_completed: bool,
}

#[derive(Deserialize)]
enum TodoUpdate {
    SetStatus(bool),
    ChangeName(String),
}

async fn get_todos(State(pool): State<SqlitePool>) -> Json<Value> {
    let todos: Vec<Todo> = sqlx::query_as("select * from todos")
	.fetch_all(&pool).await.unwrap();
    Json(json!(todos))
}

async fn add_todo(
    State(pool): State<SqlitePool>,
    Json(body): Json<Todo>,
) {
    sqlx::query("insert into todos values (?, ?, ?)")
	.bind(&body.id)
	.bind(&body.name)
	.bind(&body.is_completed)
	.execute(&pool).await.unwrap();
}

async fn update_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i8>,
    Json(body): Json<TodoUpdate>,
) {
    match body {
	TodoUpdate::SetStatus(val) => sqlx::query("update todos set is_completed=? where id=?")
	    .bind(val)
	    .bind(id)
	    .execute(&pool).await.unwrap(),
	TodoUpdate::ChangeName(val) => sqlx::query("update todos set name=? where id=?")
	    .bind(val)
	    .bind(id)
	    .execute(&pool).await.unwrap(),
    };
}

async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i8>,
) {
    sqlx::query("delete from todos where id=?")
	.bind(id)
	.execute(&pool).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("todos.db").await?;
    
    let app = Router::new()
	.route("/", get(get_todos))
	.route("/", post(add_todo))
	.route("/:id", patch(update_todo))
	.route("/:id", delete(delete_todo))
        .with_state(pool);
    
    let listener = tokio::net::TcpListener
	::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}
