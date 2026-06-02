use crate::{
    auth,
    db::DbPool,
    models::{Admin, Person, SocialMedia},
    queries::{
        CreatePersonPayload, CreateSocialMedia, LoginResponse, UpdatePersonPayload,
        UpdateSocialMediaPayload,
    },
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

/// Updates a person's information in the database.
pub async fn update_person(pool: &DbPool, payload: UpdatePersonPayload) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query("UPDATE persons SET admin_id = $1, name = $2, surname = $3, sex = $4, birthdate = $5, birthplace = $6, class = $7, number = $8, parent_name = $9, parent_number = $10, email = $11 WHERE id = $12")
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
                .bind(payload.id)
                .execute(p)
                .await?;
            Ok(())
        }
        DbPool::Sqlite(p) => {
            sqlx::query("UPDATE persons SET admin_id = $1, name = $2, surname = $3, sex = $4, birthdate = $5, birthplace = $6, class = $7, number = $8, parent_name = $9, parent_number = $10, email = $11 WHERE id = $12")
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
                .bind(payload.id)
                .execute(p)
                .await?;
            Ok(())
        }
    }
}

/// Creates a new social media entry in the database.
pub async fn create_social_media(
    pool: &DbPool,
    payload: CreateSocialMedia,
) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query(
                "INSERT INTO social_medias (person_id, platform, pseudo) VALUES ($1, $2, $3)",
            )
            .bind(payload.person_id)
            .bind(payload.platform)
            .bind(payload.pseudo)
            .execute(p)
            .await?;
            Ok(())
        }
        DbPool::Sqlite(p) => {
            sqlx::query(
                "INSERT INTO social_medias (person_id, platform, pseudo) VALUES ($1, $2, $3)",
            )
            .bind(payload.person_id)
            .bind(payload.platform)
            .bind(payload.pseudo)
            .execute(p)
            .await?;
            Ok(())
        }
    }
}

/// Retrieves all social media entries for a given person from the database.
pub async fn get_social_medias(
    pool: &DbPool,
    person_id: i64,
) -> Result<Vec<SocialMedia>, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query_as::<_, SocialMedia>("SELECT * FROM social_medias WHERE person_id = $1")
                .bind(person_id)
                .fetch_all(p)
                .await
        }
        DbPool::Sqlite(p) => {
            sqlx::query_as::<_, SocialMedia>("SELECT * FROM social_medias WHERE person_id = ?")
                .bind(person_id)
                .fetch_all(p)
                .await
        }
    }
}

/// Retrieves a social media by their ID from the database.
pub async fn get_social_media(pool: &DbPool, id: i64) -> Result<Option<SocialMedia>, sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query_as::<_, SocialMedia>("SELECT * FROM social_medias WHERE id = $1")
                .bind(id)
                .fetch_optional(p)
                .await
        }
        DbPool::Sqlite(p) => {
            sqlx::query_as::<_, SocialMedia>("SELECT * FROM social_medias WHERE id = ?")
                .bind(id)
                .fetch_optional(p)
                .await
        }
    }
}

/// Deletes a social media by their ID from the database.
pub async fn delete_social_media(pool: &DbPool, id: i64) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query("DELETE FROM social_medias WHERE id = $1")
                .bind(id)
                .execute(p)
                .await?;
            Ok(())
        }
        DbPool::Sqlite(p) => {
            sqlx::query("DELETE FROM social_medias WHERE id = ?")
                .bind(id)
                .execute(p)
                .await?;
            Ok(())
        }
    }
}

/// Updates a social media information in the database.
pub async fn update_social_media(
    pool: &DbPool,
    payload: UpdateSocialMediaPayload,
) -> Result<(), sqlx::Error> {
    match pool {
        DbPool::Postgres(p) => {
            sqlx::query("UPDATE social_medias SET platform = $1, pseudo = $2 WHERE id = $3")
                .bind(payload.platform)
                .bind(payload.pseudo)
                .bind(payload.id)
                .execute(p)
                .await?;
            Ok(())
        }
        DbPool::Sqlite(p) => {
            sqlx::query("UPDATE social_medias SET platform = $1, pseudo = $2 WHERE id = $3")
                .bind(payload.platform)
                .bind(payload.pseudo)
                .bind(payload.id)
                .execute(p)
                .await?;
            Ok(())
        }
    }
}
