use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use crate::manager::app::events::{AppEvent, AppEventHandler};

#[derive(Debug, Default)]
pub struct LogHandler;

#[async_trait]
impl AppEventHandler for LogHandler {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &*event {
            AppEvent::Start { .. } => {
                info!("app started")
            }
            AppEvent::End {
                application: _,
                result,
            } => match &**result {
                Ok(status) => {
                    info!(?status)
                }
                Err(error) => {
                    info!(?error)
                }
            },
        }

        Ok(())
    }
}
