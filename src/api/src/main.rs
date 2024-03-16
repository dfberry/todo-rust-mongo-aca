//! Provides a RESTful web server managing some lists.
//!
//! API will be:
//!
//! - `GET /lists`: return a JSON list of lists.
//! - `POST /lists`: create a new list.
//! - `PATCH /lists/:id`: update a specific list.
//! - `DELETE /lists/:id`: delete a specific list.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-lists
//! ```

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_lists=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = Db::default();

    // Compose the routes
    let app = Router::new()
        .route("/lists", get(lists_index).post(lists_create))
        // Add middleware to all routes
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
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db);



    let listener = tokio::net::TcpListener::bind("127.0.0.1:3100")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    tracing::debug!("route: lists");
    axum::serve(listener, app).await.unwrap();
}

// The query parameters for lists index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

async fn lists_index(
    pagination: Option<Query<Pagination>>,
    State(db): State<Db>,
) -> impl IntoResponse {
    let lists = db.read().unwrap();

    let Query(pagination) = pagination.unwrap_or_default();

    let lists = lists
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(lists)
}

#[derive(Debug, Deserialize)]
struct Createlist {
    text: String,
}

async fn lists_create(State(db): State<Db>, Json(input): Json<Createlist>) -> impl IntoResponse {
    let list = list {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    db.write().unwrap().insert(list.id, list.clone());

    (StatusCode::CREATED, Json(list))
}

#[derive(Debug, Deserialize)]
struct Updatelist {
    text: Option<String>,
    completed: Option<bool>,
}

async fn lists_update(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
    Json(input): Json<Updatelist>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut list = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        list.text = text;
    }

    if let Some(completed) = input.completed {
        list.completed = completed;
    }

    db.write().unwrap().insert(list.id, list.clone());

    Ok(Json(list))
}

async fn lists_delete(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

type Db = Arc<RwLock<HashMap<Uuid, list>>>;

#[derive(Debug, Serialize, Clone)]
struct list {
    id: Uuid,
    text: String,
    completed: bool,
}