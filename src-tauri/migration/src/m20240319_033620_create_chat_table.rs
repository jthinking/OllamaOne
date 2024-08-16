use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chat::Title).string().not_null())
                    .col(ColumnDef::new(Chat::Model).string().not_null())
                    .col(ColumnDef::new(Chat::Plugin).string())
                    .col(ColumnDef::new(Chat::Shared).boolean().not_null())
                    .col(ColumnDef::new(Chat::Marked).boolean().not_null())
                    .col(ColumnDef::new(Chat::Messages).string())
                    .col(ColumnDef::new(Chat::CreateTime).big_integer().not_null())
                    .col(ColumnDef::new(Chat::UpdateTime).big_integer().not_null())
                    .col(ColumnDef::new(Chat::Status).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Chat {
    Table,
    Id,         // ID
    Title,      // 对话标题
    Model,      // 模型ID
    Plugin,     // 插件ID
    Shared,     // 是否已分享
    Marked,     // 是否已收藏
    Messages,   // 对话消息列表
    CreateTime, // 创建时间
    UpdateTime, // 更新时间
    Status,     // 状态
}
