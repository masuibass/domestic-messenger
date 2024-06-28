use sea_orm_migration::prelude::*;

mod m20220101_000001_create_member_table;
mod m20220101_000002_create_message_table;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_member_table::Migration),
            Box::new(m20220101_000002_create_message_table::Migration),
        ]
    }
}
