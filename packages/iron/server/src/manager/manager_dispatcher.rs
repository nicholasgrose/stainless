use std::sync::Arc;

use async_trait::async_trait;

use crate::manager::app::events::{AppEvent, AppEventDispatcher};
use crate::manager::ApplicationManager;

#[derive(Debug)]
pub struct ManagerDispatcher {
    pub manager: Arc<ApplicationManager>,
}

#[async_trait]
impl AppEventDispatcher for ManagerDispatcher {
    async fn dispatch(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &*event {
            AppEvent::Start { .. } => {}
            AppEvent::End { application, .. } => {
                let app_id = &application.read().await.properties.id;

                self.manager.remove(app_id).await;
            }
        }

        Ok(())
    }

    fn dispatch_sync(&self, _event: Arc<AppEvent>) -> anyhow::Result<()> {
        Ok(())
    }
}
