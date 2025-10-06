use sea_orm::{DatabaseConnection, DbErr, Statement};
use sea_orm::ConnectionTrait;

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    log::info!("Running database migrations...");

    let users_table = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    "#;

    let persons_table = r#"
        CREATE TABLE IF NOT EXISTS persons (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            age INTEGER NOT NULL,
            email TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    "#;

    let videos_table = r#"
        CREATE TABLE IF NOT EXISTS videos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            url TEXT NOT NULL,
            author_id INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (author_id) REFERENCES persons(id) ON DELETE CASCADE
        )
    "#;

    db.execute(Statement::from_string(
        db.get_database_backend(),
        users_table.to_string(),
    ))
    .await?;

    db.execute(Statement::from_string(
        db.get_database_backend(),
        persons_table.to_string(),
    ))
    .await?;

    db.execute(Statement::from_string(
        db.get_database_backend(),
        videos_table.to_string(),
    ))
    .await?;

    log::info!("Database migrations completed successfully");
    Ok(())
}
