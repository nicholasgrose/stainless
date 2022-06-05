use diesel::{
    deserialize::{self, FromSqlRow},
    sqlite::Sqlite,
    Queryable, Selectable,
};
use juniper::{GraphQLObject, GraphQLUnion};

use crate::database::schema;

use self::papermc::PaperMC;

pub mod papermc;

#[derive(GraphQLObject, Queryable)]
pub struct Minecraft {
    pub jvm_runtime_arguments: Vec<String>,
    pub game_version: String,
    #[diesel(embed)]
    pub server: MinecraftServer,
}

#[derive(GraphQLUnion)]
pub enum MinecraftServer {
    PaperMC(PaperMC),
}

impl Queryable<schema::papermc_servers::SqlType, Sqlite> for MinecraftServer
where
    (String, String, String, i32): FromSqlRow<schema::papermc_servers::SqlType, Sqlite>,
{
    type Row = (String, String, String, i32);

    fn build(
        (_name, _minecraft_server_type, project, build): Self::Row,
    ) -> deserialize::Result<Self> {
        Ok(MinecraftServer::PaperMC(PaperMC {
            build,
            project,
        }))
    }
}
