use crate::models::Admin;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize)]
pub struct AdminQuery {
    pub api_key: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CreateAdminPayload {
    #[schema(example = "admin")]
    pub name: String,
    #[schema(example = "root")]
    pub password: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CreateSocialMedia {
    #[schema(example = 1)]
    pub person_id: i64,
    #[schema(example = "facebook")]
    pub platform: String,
    #[schema(example = "woumo")]
    pub pseudo: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, ToSchema)]
pub struct LoginPayload {
    #[schema(example = "admin")]
    pub name: String,
    #[schema(example = "root")]
    pub password: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CreatePersonPayload {
    #[schema(example = 1)]
    pub admin_id: i64,
    #[schema(example = "Chaltouang Woumo")]
    pub name: String,
    #[schema(example = "Manasse")]
    pub surname: String,
    #[schema(example = "M")]
    pub sex: String,
    #[schema(example = "1990-01-01")]
    pub birthdate: String,
    #[schema(example = "Yaounde")]
    pub birthplace: String,
    #[schema(example = "Terminal S")]
    pub class: String,
    #[schema(example = "+237 690 00 00 00")]
    pub number: String,
    #[schema(example = "Woumo Jean")]
    pub parent_name: String,
    #[schema(example = "+237 690 00 50 00")]
    pub parent_number: String,
    #[schema(example = "email@example.com")]
    pub email: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, ToSchema)]
pub struct UpdatePersonPayload {
    #[schema(example = 56)]
    pub id: i64,
    #[schema(example = 1)]
    pub admin_id: i64,
    #[schema(example = "Chaltouang Woumo")]
    pub name: String,
    #[schema(example = "Manasse")]
    pub surname: String,
    #[schema(example = "M")]
    pub sex: String,
    #[schema(example = "2010-05-04")]
    pub birthdate: String,
    #[schema(example = "Yaounde")]
    pub birthplace: String,
    #[schema(example = "Terminal L")]
    pub class: String,
    #[schema(example = "+237 690 50 02 67")]
    pub number: String,
    #[schema(example = "Woumo Jean")]
    pub parent_name: String,
    #[schema(example = "+237 690 23 50 00")]
    pub parent_number: String,
    #[schema(example = "email@example.com")]
    pub email: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, ToSchema)]
pub struct UpdateSocialMediaPayload {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "Facebook")]
    pub platform: String,
    #[schema(example = "chaltouangwoumo")]
    pub pseudo: String,
    #[schema(example = 1)]
    pub admin_id: i64,
    #[schema(example = 56)]
    pub person_id: i64,
}

pub enum LoginResponse {
    Admin(Admin),
    NotFound,
    Unauthorized,
}
