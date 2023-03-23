use sea_orm::*;

const DATABASE_URL: &'static str = "postgres://postgres:pass@localhost:5432/default_db";

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}
