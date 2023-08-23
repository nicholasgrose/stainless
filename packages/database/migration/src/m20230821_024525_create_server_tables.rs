use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_server_table(manager).await?;
        self.create_executable_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Executable::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ServerHost::Table).to_owned())
            .await
    }
}

impl Migration {
    async fn create_server_table<'a>(&self, manager: &SchemaManager<'a>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServerHost::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServerHost::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServerHost::Name).string().not_null())
                    .col(ColumnDef::new(ServerHost::Address).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn create_executable_table<'a>(&self, manager: &SchemaManager<'a>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Executable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Executable::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Executable::Server).uuid().not_null())
                    .col(ColumnDef::new(Executable::Name).string().not_null())
                    .col(ColumnDef::new(Executable::Command).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Executable::Table, Executable::Server)
                            .to(ServerHost::Table, ServerHost::Id),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum ServerHost {
    Table,
    Id,
    Name,
    Address,
}

#[derive(Iden)]
pub enum Executable {
    Table,
    Id,
    Name,
    Server,
    Command,
}
