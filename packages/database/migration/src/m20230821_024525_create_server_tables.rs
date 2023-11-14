use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_server_table(manager).await?;
        self.create_app_args_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Application::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AppArgs::Table).to_owned())
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
                    .col(ColumnDef::new(Application::Command).uuid().not_null())
                    .col(ColumnDef::new(Application::Active).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn create_app_args_table<'a>(&self, manager: &SchemaManager<'a>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AppArgs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AppArgs::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AppArgs::AppId).uuid().not_null())
                    .col(ColumnDef::new(AppArgs::Argument).string().not_null())
                    .col(ColumnDef::new(AppArgs::NextArg).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .from(AppArgs::Table, AppArgs::AppId)
                            .to(Application::Table, Application::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AppArgs::Table, AppArgs::NextArg)
                            .to(AppArgs::Table, AppArgs::Id),
                    )
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
    Active,
}

#[derive(Iden)]
pub enum AppArgs {
    Table,
    Id,
    AppId,
    Argument,
    NextArg,
}
