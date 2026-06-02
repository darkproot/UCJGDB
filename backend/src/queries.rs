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

pub enum LoginResponse {
    Admin(Admin),
    NotFound,
    Unauthorized,
}
