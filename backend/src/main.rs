mod handlers;
mod models;
mod repository;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::create_admin_handler,
        handlers::all_admins,
        handlers::all_persons,
        handlers::all_social_media,
    ),
    components(schemas(models::Admin, models::Person, models::SocialMedia))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let ip_addr = [0, 0, 0, 0];
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let port: u16 = port.parse().unwrap_or(3000);
    let addr = SocketAddr::from((ip_addr, port));

    let database_url = env::var("DATABASE_URL").unwrap_or("sqlite://dev.db".to_string());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/admins", get(handlers::all_admins))
        .route("/persons", get(handlers::all_persons))
        .route("/social-medias", get(handlers::all_social_media))
        .route("/admin", post(handlers::create_admin_handler))
        .merge(SwaggerUi::new("/about").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);

    let addr = SocketAddr::from(addr);
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
