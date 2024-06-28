pub mod entities;
mod migrator;

pub use entities::prelude::{Member, Message};
pub use sea_orm::{
    entity::{EntityTrait, Set},
    DbConn, DbErr,
};

use migrator::Migrator;
use sea_orm::Database;
use sea_orm_migration::{MigratorTrait, SchemaManager};

pub async fn connect(database_url: &str) -> Result<DbConn, DbErr> {
    Database::connect(database_url).await
}

pub async fn refresh(conn: &DbConn) -> Result<(), DbErr> {
    Migrator::refresh(conn).await?;
    let schema_manager = SchemaManager::new(conn);
    assert!(schema_manager.has_table("member").await?);
    assert!(schema_manager.has_table("message").await?);
    Ok(())
}
