use std::fmt::Debug;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, TransactionTrait};
use tracing::{info, instrument};

use crate::database::IronDatabase;

#[async_trait]
pub trait Insert: Debug + Send + Sync {
    async fn insert(&self, connection: &impl ConnectionTrait) -> anyhow::Result<()>;
}

pub trait InsertModel<T, C>
where
    T: ActiveModelTrait,
    C: Debug,
{
    fn build_model(&self, context: &C) -> anyhow::Result<T>;
}

impl IronDatabase {
    #[instrument]
    pub async fn insert(&self, value: &impl Insert) -> anyhow::Result<()> {
        info!("inserting new entry");

        let transaction = self.connection.begin().await?;

        value.insert(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}
