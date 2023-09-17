use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::manager::app::events::{AppEvent, AppEventDispatcher};

#[derive(Debug, Default)]
pub struct LogDispatcher;

#[async_trait]
impl AppEventDispatcher for LogDispatcher {
    #[instrument]
    async fn dispatch(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
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
