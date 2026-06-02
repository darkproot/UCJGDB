use crate::auth;
use crate::db::DbPool;
use crate::models::{Admin, Person, SocialMedia};
use crate::queries::{
    AdminQuery, CreateAdminPayload, CreatePersonPayload, LoginPayload, LoginResponse,
    UpdatePersonPayload,
};
use crate::repository;
use axum::extract::Path;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use std::env;

/// Handler for logging in an admin.
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Connexion réussie", body = Admin),
        (status = 400, description = "Mauvaise requete"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Admin not found"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn login(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Admin>, (StatusCode, String)> {
    let name = payload.name;
    let password = payload.password;
    match repository::login(&pool, name.as_str(), password.as_str()).await {
        Ok(LoginResponse::Admin(admin)) => Ok(Json(admin)),
        Ok(LoginResponse::Unauthorized) => {
            Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()))
        }
        Ok(LoginResponse::NotFound) => Err((StatusCode::NOT_FOUND, "Admin not found".to_string())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

/// Handler for retrieving all admins from the database.
#[utoipa::path(
    get,
    path = "/admins",
    params(
        ("api_key" = String, Query, description = "La clef d'API pour plus de securite")
    ),
    responses(
        (status = 200, description = "Liste des administrateurs récupérée avec succès", body = [Admin]),
        (status = 500, description = "Erreur interne du serveur lors de la lecture en base de données", body = String)
    )
)]
pub async fn all_admins(
    State(pool): State<DbPool>,
    Query(params): Query<AdminQuery>,
) -> Result<Json<Vec<Admin>>, (StatusCode, String)> {
    dotenvy::dotenv().ok();

    if params.api_key.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Need the API key.".to_string()));
    } else if params.api_key != env::var("API_KEY").unwrap() {
        return Err((StatusCode::UNAUTHORIZED, "Wrong API key..".to_string()));
    }

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
    State(pool): State<DbPool>,
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
    State(pool): State<DbPool>,
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
    request_body = CreateAdminPayload,
    responses(
        (status = 201, description = "Administrateur créé avec succès"),
        (status = 400, description = "Mauvaise requete"),
        (status = 409, description = "Admin déjà existant"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn create_admin_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateAdminPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let name = payload.name;
    let password = auth::hash_password(payload.password.as_str()).unwrap();

    if name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Missing the field name".to_string(),
        ));
    }
    if password.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Missing the field password".to_string(),
        ));
    }

    if repository::check_admin(&pool, name.as_str())
        .await
        .unwrap_or(false)
    {
        return Err((StatusCode::CONFLICT, "Admin already exists".to_string()));
    }

    match repository::create_admin(&pool, name, password).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            // En cas d'erreur de base de données (ex: contrainte violée)
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

/// Handler for getting an admin by their ID from the database.
#[utoipa::path(
    get,
    path = "/admin/{id}",
    params(
        ("id" = i64, Path, description = "ID de l'administrateur"),
    ),
    responses(
        (status = 200, description = "Administrateur récupéré avec succès"),
        (status = 404, description = "Admin non trouvé"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn get_admin_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<Json<Admin>, (StatusCode, String)> {
    match repository::get_admin(&pool, id).await {
        Ok(admin) => {
            if let Some(admin) = admin {
                Ok(Json(admin))
            } else {
                Err((StatusCode::NOT_FOUND, "Admin not found".to_string()))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler for deleting an admin by their ID from the database.
#[utoipa::path(
    delete,
    path = "/admin/{id}",
    params(
        ("id" = i64, Path, description = "ID de l'administrateur"),
    ),
    responses(
        (status = 200, description = "Administrateur supprimé avec succès"),
        (status = 404, description = "Admin non trouvé"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn delete_admin_handler(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<(), (StatusCode, String)> {
    if let Err(e) = repository::get_admin(&pool, id).await {
        return Err((StatusCode::NOT_FOUND, e.to_string()));
    }

    match repository::delete_admin(&pool, id).await {
        Ok(res) => {
            if res {
                Ok(())
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete admin".to_string(),
                ))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler for creating a person in the database.
#[utoipa::path(
    post,
    path = "/person",
    request_body = CreatePersonPayload,
    responses(
        (status = 201, description = "Personne créée avec succès"),
        (status = 400, description = "Mauvaise requete"),
        (status = 404, description = "Admin non trouvé"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn create_person_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<CreatePersonPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    let admin = repository::get_admin(&pool, payload.admin_id).await;

    // Check if the admin exists before creating the person
    if let Ok(Some(_)) = admin {
        let res = repository::create_person(&pool, payload).await;
        if let Err(e) = res {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        } else {
            Ok(StatusCode::CREATED)
        }
    } else {
        Err((StatusCode::NOT_FOUND, "Admin not found".to_string()))
    }
}

/// Handler for getting a person by their ID from the database.
#[utoipa::path(
    get,
    path = "/person/{id}/{admin_id}",
    params(
        ("id" = i64, Path, description = "ID de la personne"),
        ("admin_id" = i64, Path, description = "ID de l'administrateur"),
    ),
    responses(
        (status = 200, description = "Personne récupéré avec succès"),
        (status = 404, description = "Personne non trouvé"),
        (status = 401, description = "Admin non trouvé"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn get_person_handler(
    State(pool): State<DbPool>,
    Path((id, admin_id)): Path<(i64, i64)>,
) -> Result<Json<Person>, (StatusCode, String)> {
    match repository::get_admin(&pool, admin_id).await {
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        Ok(None) => Err((StatusCode::UNAUTHORIZED, "Admin introuvable".to_string())),
        Ok(Some(_)) => {
            let person = repository::get_person(&pool, id).await;
            match person {
                Ok(Some(res)) => Ok(Json(res)),
                Ok(None) => Err((StatusCode::NOT_FOUND, "Person not found".to_string())),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
    }
}

/// Handler for deleting a person by their ID from the database.
#[utoipa::path(
    delete,
    path = "/person/{id}/{admin_id}",
    params(
        ("id" = i64, Path, description = "ID de la personne"),
        ("admin_id" = i64, Path, description = "ID de l'administrateur"),
    ),
    responses(
        (status = 200, description = "Personne supprimée avec succès"),
        (status = 404, description = "Personne non trouvée"),
        (status = 403, description = "Accès refusé"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn delete_person_handler(
    State(pool): State<DbPool>,
    Path((id, admin_id)): Path<(i64, i64)>,
) -> Result<StatusCode, (StatusCode, String)> {
    match repository::get_admin(&pool, admin_id).await {
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        Ok(None) => Err((StatusCode::UNAUTHORIZED, "Admin introuvable".to_string())),
        Ok(Some(_)) => match repository::get_person(&pool, id).await {
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            Ok(None) => Err((StatusCode::NOT_FOUND, "Person not found".to_string())),
            Ok(Some(_)) => {
                let res = repository::delete_person(&pool, id).await;
                match res {
                    Ok(_) => Ok(StatusCode::OK),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
                }
            }
        },
    }
}

/// Update a person's information.
#[utoipa::path(
    put,
    path = "/person",
    request_body = UpdatePersonPayload,
    responses(
        (status = 200, description = "Information mise à jour avec succès"),
        (status = 400, description = "Mauvaise requete"),
        (status = 404, description = "Personne non trouvée"),
        (status = 500, description = "Erreur interne du serveur")
    )
)]
pub async fn update_person_handler(
    State(pool): State<DbPool>,
    Json(payload): Json<UpdatePersonPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    match repository::get_admin(&pool, payload.admin_id).await {
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        Ok(None) => Err((StatusCode::UNAUTHORIZED, "Admin introuvable".to_string())),
        Ok(Some(_)) => match repository::get_person(&pool, payload.id).await {
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            Ok(None) => Err((StatusCode::NOT_FOUND, "Person not found".to_string())),
            Ok(Some(_)) => {
                let res = repository::update_person(&pool, payload).await;
                match res {
                    Ok(_) => Ok(StatusCode::OK),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
                }
            }
        },
    }
}
