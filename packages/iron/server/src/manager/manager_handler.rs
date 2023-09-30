use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use async_trait::async_trait;

use crate::manager::app::events::{AppEvent, AppEventHandler, AppEventType};
use crate::manager::ApplicationManager;

pub struct ManagerHandler {
    pub manager: Arc<ApplicationManager>,
}

impl Debug for ManagerHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ManagerHandler))
    }
}

#[async_trait]
impl AppEventHandler for ManagerHandler {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &event.event_type {
            AppEventType::Start { .. } => {}
            AppEventType::End { .. } => {
                let app_id = &event.application.config.properties.id;

                self.manager.remove(app_id).await;
            }
            AppEventType::Print { .. } => {}
        }

        Ok(())
    }
}
