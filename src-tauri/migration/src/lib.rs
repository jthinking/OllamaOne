pub use sea_orm_migration::prelude::*;

mod m20240319_033338_create_config_table;
mod m20240319_033620_create_chat_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240319_033338_create_config_table::Migration),
            Box::new(m20240319_033620_create_chat_table::Migration),
        ]
    }
}
