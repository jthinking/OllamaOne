use ::entity::{chat, chat::Entity as Chat};
use sea_orm::*;

pub struct ChatQuery;

impl ChatQuery {
    pub async fn find_chat_by_id(db: &DbConn, id: i32) -> Result<Option<chat::Model>, DbErr> {
        Chat::find_by_id(id).one(db).await
    }

    /// If ok, returns (chat models, num pages).
    pub async fn find_chats_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<chat::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Chat::find()
            .select_only()
            .columns([
                chat::Column::Id,
                chat::Column::Title,
                chat::Column::Model,
                chat::Column::Plugin,
                chat::Column::Shared,
                chat::Column::Marked,
                chat::Column::CreateTime,
                chat::Column::UpdateTime,
                chat::Column::Status,
            ])
            .order_by_desc(chat::Column::UpdateTime)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
