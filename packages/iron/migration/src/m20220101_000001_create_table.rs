use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(ServerConfig::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ServerConfig::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(ServerConfig::Name).string().not_null())
                .col(
                    ColumnDef::new(ServerConfig::Type)
                        .enumeration(
                            ServerConfig::Type,
                            vec![MinecraftServer::Table],
                        )
                        .not_null()
                )
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(MinecraftServer::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(MinecraftServer::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                )
                .col(ColumnDef::new(MinecraftServer::GameVersion).string().not_null())
                .col(
                    ColumnDef::new(MinecraftServer::ServerType)
                        .enumeration(
                            MinecraftServer::ServerType,
                            vec![PaperMcServer::Table],
                        )
                        .not_null()
                )
                .foreign_key(ForeignKey::create()
                    .from(MinecraftServer::Table, MinecraftServer::Id)
                    .to(ServerConfig::Table, ServerConfig::Id)
                )
                .to_owned(),
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(MinecraftJvmArgument::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MinecraftJvmArgument::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(MinecraftJvmArgument::Argument).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(MinecraftJvmArgument::Table, MinecraftJvmArgument::Id)
                            .to(MinecraftServer::Table, MinecraftServer::Id)
                    )
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(PaperMcServer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaperMcServer::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                    )
                    .col(ColumnDef::new(PaperMcServer::Build).unsigned().not_null())
                    .col(ColumnDef::new(PaperMcServer::Project).string().not_null())
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ServerConfig::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(MinecraftServer::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(MinecraftJvmArgument::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(PaperMcServer::Table).to_owned()).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ServerConfig {
    Table,
    Id,
    Name,
    Type,
}

#[derive(Iden)]
enum MinecraftServer {
    Table,
    Id,
    GameVersion,
    ServerType,
}

#[derive(Iden)]
enum MinecraftJvmArgument {
    Table,
    Id,
    Argument,
}

#[derive(Iden)]
enum PaperMcServer {
    Table,
    Id,
    Project,
    Build,
}
