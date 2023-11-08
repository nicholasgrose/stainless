use crate::manager::app::events::{AppEvent, AppEventHandler};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct PaperMcHandler;

#[async_trait]
impl AppEventHandler for PaperMcHandler {
    async fn handle(&self, _event: Arc<AppEvent>) -> anyhow::Result<()> {
        Ok(())
    }
}
