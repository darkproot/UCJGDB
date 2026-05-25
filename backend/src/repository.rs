use crate::models::{Admin, Person, SocialMedia};
use sqlx::SqlitePool;

/// Returns all admins from the database.
pub async fn get_all_admins(pool: &SqlitePool) -> Result<Vec<Admin>, sqlx::Error> {
    sqlx::query_as::<_, Admin>("SELECT * FROM admins")
        .fetch_all(pool)
        .await
}

/// Returns all people from the database.
pub async fn get_all_people(pool: &SqlitePool) -> Result<Vec<Person>, sqlx::Error> {
    sqlx::query_as::<_, Person>("SELECT * FROM persons")
        .fetch_all(pool)
        .await
}

/// Returns all social media from the database.
pub async fn get_all_social_media(pool: &SqlitePool) -> Result<Vec<SocialMedia>, sqlx::Error> {
    sqlx::query_as::<_, SocialMedia>("SELECT * FROM social_medias")
        .fetch_all(pool)
        .await
}

/// Creates a new admin in the database.
pub async fn create_admin(pool: &SqlitePool, admin: Admin) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO admins (name, password) VALUES (?, ?)",
        admin.name,
        admin.password
    )
    .execute(pool)
    .await
    .map(|_| ())
}
