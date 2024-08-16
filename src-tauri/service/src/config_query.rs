use ::entity::{config, config::Entity as Config};
use sea_orm::*;

pub struct ConfigQuery;

impl ConfigQuery {
    pub async fn find_config_by_id(
        db: &DbConn,
        id: String,
    ) -> Result<Option<config::Model>, DbErr> {
        Config::find_by_id(id).one(db).await
    }

    /// If ok, returns (config models, num pages).
    pub async fn find_configs_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<config::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Config::find()
            .order_by_asc(config::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated posts
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
