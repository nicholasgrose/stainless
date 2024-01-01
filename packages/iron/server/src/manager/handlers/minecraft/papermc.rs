use std::sync::Arc;

use crate::manager::app::events::{AppEvent, AsyncAppEventHandler};

#[derive(Default, Debug)]
pub struct PaperMcHandler;

impl AsyncAppEventHandler for PaperMcHandler {
    async fn handle_async(&self, _event: Arc<AppEvent>) -> anyhow::Result<()> {
        Ok(())
    }
}
