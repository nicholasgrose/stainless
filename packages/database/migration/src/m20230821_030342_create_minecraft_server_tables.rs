use sea_orm_migration::prelude::*;

use crate::m20230821_024525_create_server_tables::Application;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_minecraft_server_table(manager).await?;
        self.create_papermc_server_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MinecraftServer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PaperMcServer::Table).to_owned())
            .await
    }
}

impl Migration {
    async fn create_minecraft_server_table<'a>(
        &self,
        manager: &SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MinecraftServer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MinecraftServer::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MinecraftServer::GameVersion)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MinecraftServer::Table, MinecraftServer::Id)
                            .to(Application::Table, Application::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn create_papermc_server_table<'a>(
        &self,
        manager: &SchemaManager<'a>,
    ) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaperMcServer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaperMcServer::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PaperMcServer::Build).unsigned().not_null())
                    .col(ColumnDef::new(PaperMcServer::Project).string().not_null())
                    .col(
                        ColumnDef::new(PaperMcServer::BuildUpdateOff)
                            .boolean()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PaperMcServer::Table, PaperMcServer::Id)
                            .to(MinecraftServer::Table, MinecraftServer::Id),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum MinecraftServer {
    Table,
    Id,
    GameVersion,
}

#[derive(Iden)]
enum PaperMcServer {
    Table,
    Id,
    Project,
    Build,
    BuildUpdateOff,
}
