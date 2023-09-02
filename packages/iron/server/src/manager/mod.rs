use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::manager::app::control::start;
use crate::manager::app::{AppCreationSettings, Application};
use crate::manager::manager_dispatcher::ManagerDispatcher;

pub mod app;
mod log_dispatcher;
mod manager_dispatcher;

#[derive(Debug, Default)]
pub struct ApplicationManager {
    applications: RwLock<HashMap<Uuid, Arc<RwLock<Application>>>>,
}

impl ApplicationManager {
    #[instrument(skip(self))]
    pub async fn execute_new(&self, app_settings: AppCreationSettings) -> anyhow::Result<()> {
        info!("starting application");

        let app_id = app_settings.properties.id;
        let app = Application::new(app_settings);
        app.subscribe_dispatcher(Box::<ManagerDispatcher>::default());

        let mut apps = self.applications.write().await;

        apps.insert(app_id, RwLock::new(app).into());

        if let Some(app) = apps.get(&app_id) {
            start(app).await?;
        }

        Ok(())
    }
}
