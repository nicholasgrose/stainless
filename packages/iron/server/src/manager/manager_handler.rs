use std::sync::Arc;

use async_trait::async_trait;

use crate::manager::app::events::{AppEvent, AppEventHandler};
use crate::manager::ApplicationManager;

#[derive(Debug)]
pub struct ManagerHandler {
    pub manager: Arc<ApplicationManager>,
}

#[async_trait]
impl AppEventHandler for ManagerHandler {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &*event {
            AppEvent::Start { .. } => {}
            AppEvent::End { application, .. } => {
                let app_id = &application.read().await.properties.id;

                self.manager.remove(app_id).await;
            }
        }

        Ok(())
    }
}
