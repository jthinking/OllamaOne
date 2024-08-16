use ::entity::{config, config::Entity as Config};
use sea_orm::*;

pub struct ConfigMutation;

impl ConfigMutation {
    pub async fn create_config(
        db: &DbConn,
        form_data: config::Model,
    ) -> Result<config::ActiveModel, DbErr> {
        config::ActiveModel {
            id: Set(form_data.id.to_owned()),
            value: Set(form_data.value.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_config_by_id(
        db: &DbConn,
        id: String,
        form_data: config::Model,
    ) -> Result<config::Model, DbErr> {
        let config: config::ActiveModel = Config::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find config.".to_owned()))
            .map(Into::into)?;

        config::ActiveModel {
            id: config.id,
            value: Set(form_data.value.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_config(db: &DbConn, id: String) -> Result<DeleteResult, DbErr> {
        let config: config::ActiveModel = Config::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find config.".to_owned()))
            .map(Into::into)?;

        config.delete(db).await
    }

    pub async fn delete_all_configs(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Config::delete_many().exec(db).await
    }
}
