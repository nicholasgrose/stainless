use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_server_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Application::Table).to_owned())
            .await
    }
}

impl Migration {
    async fn create_server_table<'a>(&self, manager: &SchemaManager<'a>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Application::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Application::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Application::Name).string().not_null())
                    .col(ColumnDef::new(Application::Command).string().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum Application {
    Table,
    Id,
    Name,
    Command,
}
