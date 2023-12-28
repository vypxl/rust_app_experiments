use std::time::Duration;

use anyhow::{anyhow, Error, Result};
use askama::Template;
use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::{Path, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, patch},
    BoxError, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    sql::Thing,
    Surreal,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeFile,
};
use tower_livereload::LiveReloadLayer;

type Db = Surreal<Client>;

#[tokio::main]
async fn main() -> Result<()> {
    let db_address = "localhost:8000";
    let listen_address = "0.0.0.0:8080";

    let db = Surreal::new::<Ws>(db_address).await?;
    db.use_ns("test").use_db("test").await?;

    let livereload = LiveReloadLayer::new();

    let app = Router::new()
        .route("/todo", get(todo_get_all).post(todo_post))
        .route("/todo/:id", patch(todo_patch).delete(todo_delete))
        .nest_service("/", ServeFile::new("static/index.html"))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .into_inner(),
        )
        .layer(
            livereload
                .request_predicate(|req: &Request<Body>| !req.headers().contains_key("hx-request")),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();

    println!("Listening on {}", listen_address);
    axum::serve(listener, app).await?;
    Ok(())
}

enum AppError {
    Anyhow(Error),
    Surreal(surrealdb::Error),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        AppError::Anyhow(error)
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(error: surrealdb::Error) -> Self {
        AppError::Surreal(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Anyhow(error) => {
                let message = error.to_string();
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::Surreal(error) => {
                let message = error.to_string();
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };

        let body = Json(json!({ "error": message }));

        (status, body).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoRaw {
    content: String,
}

#[derive(Template)]
#[template(path = "todo_list.html")]
struct TodoListTempl {
    todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo_list_item.html")]
struct TodoListItemTempl {
    todo: Todo,
}

async fn todo_get_all(State(db): State<Db>) -> Result<impl IntoResponse, AppError> {
    let todos: Vec<Todo> = db.select("todo").await?;
    Ok(TodoListTempl { todos })
}

async fn todo_post(
    State(db): State<Db>,
    Json(input): Json<TodoRaw>,
) -> Result<impl IntoResponse, AppError> {
    let todos: Vec<Todo> = db.create("todo").content(input).await?;
    let todo = todos.into_iter().next().unwrap();

    Ok(TodoListItemTempl { todo })
}

async fn todo_patch(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(input): Json<TodoRaw>,
) -> Result<impl IntoResponse, AppError> {
    let result: Option<Todo> = db.update(("todo", id)).content(input).await?;
    let todo = result.ok_or(anyhow!("Todo item not found"))?;
    Ok(TodoListItemTempl { todo })
}

async fn todo_delete(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let result: Option<Todo> = db.delete(("todo", id)).await?;
    let todo = result.ok_or(anyhow!("Todo item not found"))?;
    Ok(axum::Json(todo))
}
