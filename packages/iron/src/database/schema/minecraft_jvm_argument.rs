//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "minecraft_jvm_argument")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub argument_id: i32,
    pub server_id: i32,
    pub argument: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::minecraft_server::Entity",
        from = "Column::ServerId",
        to = "super::minecraft_server::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    MinecraftServer,
}

impl Related<super::minecraft_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MinecraftServer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
