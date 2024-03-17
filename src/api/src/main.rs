mod item;
mod list;
mod shared;

use dotenv::dotenv;
use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

use crate::{shared::database, shared::route::create_router};

pub struct AppState {
    db: mongodb::Database,
    app_host: String,
}

pub fn get_cors(env: &str) -> Vec<HeaderValue> {
    let mut cors_origins: Vec<HeaderValue> = std::env::var("API_ALLOW_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .split(',')
        .filter_map(|s| HeaderValue::from_str(s).ok())
        .collect::<Vec<_>>();

    if env != "development" {
        let azure_origins = ["https://portal.azure.com", "https://ms.portal.azure.com"];

        let azure_origins: Vec<HeaderValue> = azure_origins
            .iter()
            .filter_map(|s| HeaderValue::from_str(s).ok())
            .collect();

        cors_origins.extend(azure_origins);
    }

    cors_origins
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // get PORT from env
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let host_name: String = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
    let environment: String =
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());

    let cors_origins: Vec<HeaderValue> = get_cors(&environment);

    // Create a CorsLayer with the parsed origins
    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let db = match database::get_database().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to get database: {}", e);
            std::process::exit(1);
        }
    };

    let mut app = create_router(Arc::new(AppState {
        db: db.clone(),
        // concatenate host and port
        app_host: host_name + ":" + &port,
    }));

    app = app.layer(cors);

    println!("🚀 Server started successfully on port {}", port);
    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
