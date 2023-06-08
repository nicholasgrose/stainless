use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GameServer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameServer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GameServer::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GameType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameType::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GameType::TypeName)
                            .enumeration(GameType::TypeName, [MinecraftServer::Table])
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("index-server-type")
                            .if_not_exists()
                            .table(GameType::Table)
                            .col(GameType::Id)
                            .col(GameType::TypeName)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .to(GameType::Table, GameType::Id)
                            .from(GameServer::Table, GameServer::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MinecraftServer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MinecraftServer::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MinecraftServer::GameType)
                            .string()
                            .not_null()
                            .default(MinecraftServer::Table.to_string()),
                    )
                    .col(
                        ColumnDef::new(MinecraftServer::GameVersion)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MinecraftServer::Table, MinecraftServer::Id)
                            .to(GameServer::Table, GameServer::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                MinecraftServer::Table,
                                (MinecraftServer::Id, MinecraftServer::GameType),
                            )
                            .to(GameType::Table, (GameType::Id, GameType::TypeName)),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MinecraftJvmArgument::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MinecraftJvmArgument::ArgumentId)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                    )
                    .col(ColumnDef::new(MinecraftJvmArgument::ServerId).integer().not_null())
                    .col(ColumnDef::new(MinecraftJvmArgument::Argument).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(MinecraftJvmArgument::Table, MinecraftJvmArgument::ServerId)
                            .to(MinecraftServer::Table, MinecraftServer::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MinecraftServerType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MinecraftServerType::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MinecraftServerType::ServerType)
                            .enumeration(MinecraftServerType::ServerType, [PaperMcServer::Table])
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("index-minecraft-server-type")
                            .table(MinecraftServerType::Table)
                            .col(MinecraftServerType::Id)
                            .col(MinecraftServerType::ServerType)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MinecraftServerType::Table, MinecraftServerType::Id)
                            .to(MinecraftServer::Table, MinecraftServer::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PaperMcServer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaperMcServer::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PaperMcServer::ServerType)
                            .string()
                            .not_null()
                            .default(PaperMcServer::Table.to_string()),
                    )
                    .col(ColumnDef::new(PaperMcServer::Build).unsigned().not_null())
                    .col(ColumnDef::new(PaperMcServer::Project).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PaperMcServer::Table, PaperMcServer::Id)
                            .to(MinecraftServer::Table, MinecraftServer::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PaperMcServer::Table, PaperMcServer::Id)
                            .to(GameServer::Table, GameServer::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PaperMcServer::Table,
                                (PaperMcServer::Id, PaperMcServer::ServerType),
                            )
                            .to(
                                MinecraftServerType::Table,
                                (MinecraftServerType::Id, MinecraftServerType::ServerType),
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GameServer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MinecraftServer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MinecraftJvmArgument::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PaperMcServer::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum GameServer {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum GameType {
    Table,
    Id,
    TypeName,
}

#[derive(Iden)]
enum MinecraftServer {
    Table,
    Id,
    GameType,
    GameVersion,
}

#[derive(Iden)]
enum MinecraftJvmArgument {
    Table,
    ServerId,
    ArgumentId,
    Argument,
}

#[derive(Iden)]
enum MinecraftServerType {
    Table,
    Id,
    ServerType,
}

#[derive(Iden)]
enum PaperMcServer {
    Table,
    ServerType,
    Id,
    Project,
    Build,
}
