mod auth;
mod db;
mod handlers;
mod models;
mod queries;
mod repository;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
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
        handlers::get_admin_handler,
        handlers::delete_admin_handler,
        handlers::login,
        handlers::create_person_handler,
        handlers::get_person_handler,
        handlers::delete_person_handler,
        handlers::update_person_handler,
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
    let pool = db::init_pool().await.unwrap();

    let app = Router::new()
        .route("/admins", get(handlers::all_admins))
        .route("/persons", get(handlers::all_persons))
        .route("/social-medias", get(handlers::all_social_media))
        .route("/admin", post(handlers::create_admin_handler))
        .route("/admin/{id}", get(handlers::get_admin_handler))
        .route("/admin/{id}", delete(handlers::delete_admin_handler))
        .route("/login", post(handlers::login))
        .route("/person", post(handlers::create_person_handler))
        .route("/person/{id}/{admin_id}", get(handlers::get_person_handler))
        .route("/person", put(handlers::update_person_handler))
        .route(
            "/person/{id}/{admin_id}",
            delete(handlers::delete_person_handler),
        )
        .merge(SwaggerUi::new("/about").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(pool);

    let addr = SocketAddr::from(addr);
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
