use std::sync::Arc;

use async_trait::async_trait;

use crate::manager::app::events::{AppEvent, AppEventHandler, AppEventType};
use crate::manager::ApplicationManager;

#[derive(Debug)]
pub struct ManagerHandler {
    pub manager: Arc<ApplicationManager>,
}

#[async_trait]
impl AppEventHandler for ManagerHandler {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &event.event_type {
            AppEventType::Start { .. } => {}
            AppEventType::End { .. } => {
                let app_id = &event.application.read().await.config.properties.id;

                self.manager.remove(app_id).await;
            }
            AppEventType::Print { .. } => {}
        }

        Ok(())
    }
}
