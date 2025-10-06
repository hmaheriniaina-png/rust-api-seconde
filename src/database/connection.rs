use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    log::info!("Connecting to database: {}", database_url);
    let db = Database::connect(database_url).await?;
    log::info!("Database connection established successfully");
    Ok(db)
}
