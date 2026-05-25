use crate::models::{Admin, Person, SocialMedia};
use crate::repository;
use axum::{Json, extract::State, http::StatusCode};
use sqlx::SqlitePool;

/// Handler for retrieving all admins from the database.
#[utoipa::path(
    get,
    path = "/admins",
    responses(
        (status = 200, description = "Liste des administrateurs récupérée avec succès", body = [Admin]),
        (status = 500, description = "Erreur interne du serveur lors de la lecture en base de données", body = String)
    )
)]
pub async fn all_admins(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Admin>>, (StatusCode, String)> {
    let admins = repository::get_all_admins(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(admins))
}

/// Handler for retrieving all persons from the database.
#[utoipa::path(
    get,
    path = "/persons",
    responses(
        (status = 200, description = "Liste des personnes récupérée avec succès", body = [SocialMedia]),
        (status = 500, description = "Erreur interne du serveur lors de la lecture en base de données", body = String)
    )
)]
pub async fn all_persons(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Person>>, (StatusCode, String)> {
    let persons = repository::get_all_people(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(persons))
}

/// Handler for retrieving all social media from the database.
#[utoipa::path(
    get,
    path = "/social-medias",
    responses(
        (status = 200, description = "Liste des réseaux sociaux récupérée avec succès", body = [SocialMedia]),
        (status = 500, description = "Erreur interne du serveur lors de la lecture en base de données", body = String)
    )
)]
pub async fn all_social_media(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<SocialMedia>>, (StatusCode, String)> {
    let social_media = repository::get_all_social_media(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(social_media))
}

/// Handler for creating a new admin in the database.
#[utoipa::path(
    post,
    path = "/admin",
    request_body = Admin,
    responses(
        (status = 201, description = "Administrateur créé avec succès"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn create_admin_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<Admin>,
) -> Result<StatusCode, (StatusCode, String)> {
    match repository::create_admin(&pool, payload).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            // En cas d'erreur de base de données (ex: contrainte violée)
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
