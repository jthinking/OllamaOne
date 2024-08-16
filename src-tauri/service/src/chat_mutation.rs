use ::entity::{chat, chat::Entity as Chat};
use sea_orm::*;

pub struct ChatMutation;

impl ChatMutation {
    pub async fn create_chat(
        db: &DbConn,
        form_data: chat::Model,
    ) -> Result<chat::ActiveModel, DbErr> {
        chat::ActiveModel {
            title: Set(form_data.title.to_owned()),
            model: Set(form_data.model.to_owned()),
            plugin: Set(form_data.plugin.to_owned()),
            shared: Set(form_data.shared.to_owned()),
            marked: Set(form_data.marked.to_owned()),
            messages: Set(form_data.messages.to_owned()),
            create_time: Set(form_data.create_time.to_owned()),
            update_time: Set(form_data.update_time.to_owned()),
            status: Set(form_data.status.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_chat_by_id(
        db: &DbConn,
        id: i32,
        form_data: chat::Model,
    ) -> Result<chat::Model, DbErr> {
        let chat: chat::ActiveModel = Chat::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find chat.".to_owned()))
            .map(Into::into)?;

        chat::ActiveModel {
            id: chat.id,
            title: Set(form_data.title.to_owned()),
            model: Set(form_data.model.to_owned()),
            plugin: Set(form_data.plugin.to_owned()),
            shared: Set(form_data.shared.to_owned()),
            marked: Set(form_data.marked.to_owned()),
            messages: Set(form_data.messages.to_owned()),
            create_time: Set(form_data.create_time.to_owned()),
            update_time: Set(form_data.update_time.to_owned()),
            status: Set(form_data.status.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_chat(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let chat: chat::ActiveModel = Chat::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find chat.".to_owned()))
            .map(Into::into)?;

        chat.delete(db).await
    }

    pub async fn delete_all_chats(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Chat::delete_many().exec(db).await
    }
}
