use std::fmt::Debug;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, TransactionTrait};
use tracing::{debug, instrument};

use crate::database::IronDatabase;

#[async_trait]
pub trait Insert<C>: Debug + Send + Sync
where
    C: Debug,
{
    async fn insert(&self, connection: &impl ConnectionTrait, context: &C) -> anyhow::Result<()>;
}

#[async_trait]
pub trait InsertModel<T, C>
where
    T: ActiveModelTrait,
    C: Debug,
{
    async fn build_model(&self, context: &C) -> anyhow::Result<T>;
}

impl IronDatabase {
    #[instrument(skip(self))]
    pub async fn insert(&self, value: &impl Insert<()>) -> anyhow::Result<()> {
        debug!("inserting new value");

        let transaction = self.connection.begin().await?;

        value.insert(&transaction, &()).await?;

        transaction.commit().await?;

        Ok(())
    }
}
