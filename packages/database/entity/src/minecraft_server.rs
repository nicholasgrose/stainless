//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "minecraft_server")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub game_version: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::application::Entity",
        from = "Column::Id",
        to = "super::application::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Application,
    #[sea_orm(has_many = "super::paper_mc_server::Entity")]
    PaperMcServer,
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

impl Related<super::paper_mc_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PaperMcServer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
