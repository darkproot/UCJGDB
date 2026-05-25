use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Admin {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "darkproot")]
    pub name: String,
    #[schema(example = "cde4545t4grfre5")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Person {
    #[schema(example = 2)]
    pub id: i64,
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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct SocialMedia {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = 2)]
    pub person_id: i64,
    #[schema(example = "Facebook")]
    pub platform: String,
    #[schema(example = "sdc45ved#")]
    pub pseudo: String,
}
