use crate::{
    auth,
    db::DbPool,
    models::{Admin, Person, SocialMedia},
    queries::{CreatePersonPayload, LoginResponse},
};

/// Logs in an admin using the provided name and password.
pub async fn login(
    pool: &DbPool,
    name: &str,
    password_plain: &str,
) -> Result<LoginResponse, sqlx::Error> {
    let admin_res = match pool {
        DbPool::Postgres(p) => {
            sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE name = $1")
                .bind(name)
                .fetch_optional(p) // On utilise fetch_optional pour ne pas crash si le nom n'existe pas
                .await
        }
        DbPool::Sqlite(p) => {
            sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE name = ?1")
                .bind(name)
                .fetch_optional(p)
                .await
        }
    };

    match admin_res {
        Err(e) => Err(e),
        Ok(res) => {
            match res {
                None => Ok(LoginResponse::NotFound), // Missing name
                Some(admin) => {
                    if auth::verify_password(password_plain, &admin.password) {
                        Ok(LoginResponse::Admin(admin)) // Success !
                    } else {
                        Ok(LoginResponse::Unauthorized) // Invalid password
                    }
                }
            }
        }
    }
}

/// Returns all admins from the database.
pub async fn get_all_admins(pool: &DbPool) -> Result<Vec<Admin>, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query_as::<_, Admin>("SELECT * FROM admins")
                .fetch_all(p)
                .await
        }
        DbPool::Sqlite(p) => {
            sqlx::query_as::<_, Admin>("SELECT * FROM admins")
                .fetch_all(p)
                .await
        }
    }
}

/// Returns all people from the database.
pub async fn get_all_people(pool: &DbPool) -> Result<Vec<Person>, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query_as::<_, Person>("SELECT * FROM persons")
                .fetch_all(p)
                .await
        }
        DbPool::Sqlite(p) => {
            sqlx::query_as::<_, Person>("SELECT * FROM persons")
                .fetch_all(p)
                .await
        }
    }
}

/// Returns all social media from the database.
pub async fn get_all_social_media(pool: &DbPool) -> Result<Vec<SocialMedia>, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query_as::<_, SocialMedia>("SELECT * FROM social_medias")
                .fetch_all(p)
                .await
        }
        DbPool::Sqlite(p) => {
            sqlx::query_as::<_, SocialMedia>("SELECT * FROM social_medias")
                .fetch_all(p)
                .await
        }
    }
}

/// Creates a new admin in the database.
pub async fn create_admin(
    pool: &DbPool,
    name: String,
    password: String,
) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO admins (name, password) VALUES ($1, $2)")
                .bind(name)
                .bind(password)
                .execute(p)
                .await?;
        }
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO admins (name, password) VALUES (?, ?)")
                .bind(name)
                .bind(password)
                .execute(p)
                .await?;
        }
    }

    Ok(())
}

/// Checks if an admin exists in the database.
pub async fn check_admin(pool: &DbPool, name: &str) -> Result<bool, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM admins WHERE name = $1)")
                    .bind(name)
                    .fetch_one(p)
                    .await?;
            Ok(exists)
        }
        DbPool::Sqlite(p) => {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM admins WHERE name = ?)")
                    .bind(name)
                    .fetch_one(p)
                    .await?;
            Ok(exists)
        }
    }
}

/// Gets an admin by their ID from the database.
pub async fn get_admin(pool: &DbPool, id: i64) -> Result<Option<Admin>, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            let admin: Option<Admin> = sqlx::query_as("SELECT * FROM admins WHERE id = $1")
                .bind(id)
                .fetch_optional(p)
                .await?;
            Ok(admin)
        }
        DbPool::Sqlite(p) => {
            let admin: Option<Admin> = sqlx::query_as("SELECT * FROM admins WHERE id = ?")
                .bind(id)
                .fetch_optional(p)
                .await?;
            Ok(admin)
        }
    }
}

/// Deletes an admin by their ID from the database.
pub async fn delete_admin(pool: &DbPool, id: i64) -> Result<bool, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query("DELETE FROM admins WHERE id = $1")
                .bind(id)
                .execute(p)
                .await?;
            Ok(true)
        }
        DbPool::Sqlite(p) => {
            sqlx::query("DELETE FROM admins WHERE id = ?")
                .bind(id)
                .execute(p)
                .await?;
            Ok(true)
        }
    }
}

/// Creates a person in the database.
pub async fn create_person(pool: &DbPool, payload: CreatePersonPayload) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query("INSERT INTO persons (admin_id, name, surname, sex, birthdate, birthplace, class, number, parent_name, parent_number, email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
                .bind(payload.admin_id)
                .bind(payload.name)
                .bind(payload.surname)
                .bind(payload.sex)
                .bind(payload.birthdate)
                .bind(payload.birthplace)
                .bind(payload.class)
                .bind(payload.number)
                .bind(payload.parent_name)
                .bind(payload.parent_number)
                .bind(payload.email)
                .execute(p)
                .await?;
            Ok(())
        }
        DbPool::Sqlite(p) => {
            sqlx::query("INSERT INTO persons (admin_id, name, surname, sex, birthdate, birthplace, class, number, parent_name, parent_number, email) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(payload.admin_id)
                .bind(payload.name)
                .bind(payload.surname)
                .bind(payload.sex)
                .bind(payload.birthdate)
                .bind(payload.birthplace)
                .bind(payload.class)
                .bind(payload.number)
                .bind(payload.parent_name)
                .bind(payload.parent_number)
                .bind(payload.email)
                .execute(p)
                .await?;
            Ok(())
        }
    }
}

/// Retrieves a person by their ID from the database.
pub async fn get_person(pool: &DbPool, id: i64) -> Result<Option<Person>, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE id = $1")
                .bind(id)
                .fetch_optional(p)
                .await
        }
        DbPool::Sqlite(p) => {
            sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE id = ?")
                .bind(id)
                .fetch_optional(p)
                .await
        }
    }
}

/// Deletes a person by their ID from the database.
pub async fn delete_person(pool: &DbPool, id: i64) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query("DELETE FROM persons WHERE id = $1")
                .bind(id)
                .execute(p)
                .await?;
            Ok(())
        }
        DbPool::Sqlite(p) => {
            sqlx::query("DELETE FROM persons WHERE id = ?")
                .bind(id)
                .execute(p)
                .await?;
            Ok(())
        }
    }
}
