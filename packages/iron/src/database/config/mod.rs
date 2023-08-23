use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::{event, Level};
use uuid::Uuid;

use entity::prelude::*;

use crate::web::schema::game::server::GameServerConfig;

pub async fn server_config(
    id: Uuid,
    connection: &DatabaseConnection,
) -> anyhow::Result<Option<GameServerConfig>> {
    let server_row = ServerHost::find_by_id(id.to_string())
        .into_json()
        .one(connection)
        .await?;

    server_row.map(|server| {
        event!(Level::INFO, "{}", server);
    });

    Ok(None)
}
