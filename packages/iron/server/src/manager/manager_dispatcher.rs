use async_trait::async_trait;
use tracing::info;

use crate::manager::app::events::{AppEvent, AppEventDispatcher};

#[derive(Debug, Default)]
pub struct ManagerDispatcher;

#[async_trait]
impl AppEventDispatcher for ManagerDispatcher {
    async fn dispatch(&self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::Start { .. } => {
                info!("manager start")
            }
            AppEvent::End { .. } => {
                info!("manager stop")
            }
        }

        Ok(())
    }
}
