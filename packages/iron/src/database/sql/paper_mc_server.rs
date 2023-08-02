//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "paper_mc_server")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub server_type: String,
    pub build: i32,
    pub project: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::game_server::Entity",
        from = "Column::Id",
        to = "super::game_server::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    GameServer,
    #[sea_orm(
        belongs_to = "super::minecraft_server::Entity",
        from = "Column::Id",
        to = "super::minecraft_server::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MinecraftServer,
    #[sea_orm(
        belongs_to = "super::minecraft_server_type::Entity",
        from = "Column::Id",
        to = "super::minecraft_server_type::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MinecraftServerType2,
    #[sea_orm(
        belongs_to = "super::minecraft_server_type::Entity",
        from = "Column::ServerType",
        to = "super::minecraft_server_type::Column::ServerType",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MinecraftServerType1,
}

impl Related<super::game_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameServer.def()
    }
}

impl Related<super::minecraft_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MinecraftServer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
