use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{PgPool, SqlitePool};
use std::env;
use std::str::FromStr;

#[derive(Clone)]
pub enum DbPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

/// Fonction pour initialiser le bon pool selon la variable d'environnement
pub async fn init_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("La variable d'environnement DATABASE_URL doit être définie");

    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        println!("Connexion à la base de données PostgreSQL...");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        Ok(DbPool::Postgres(pool))
    } else if database_url.starts_with("sqlite:") {
        println!("Connexion à la base de données SQLite...");
        let connection_options =
            SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connection_options)
            .await?;
        Ok(DbPool::Sqlite(pool))
    } else {
        panic!("Format de DATABASE_URL non supporté (doit commencer par postgres:// ou sqlite:)");
    }
}
