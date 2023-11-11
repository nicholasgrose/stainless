use crate::manager::app::events::{AppEvent, AsyncAppEventHandler};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct PaperMcHandler;

#[async_trait]
impl AsyncAppEventHandler for PaperMcHandler {
    async fn handle_async(&self, _event: Arc<AppEvent>) -> anyhow::Result<()> {
        Ok(())
    }
}
