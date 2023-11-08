use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::manager::app::create::AppCreationSettings;
use crate::manager::app::Application;
use crate::manager::handlers::log_handler::LogHandler;
use crate::manager::handlers::manager_handler::ManagerHandler;

pub mod app;
pub mod handlers;

#[derive(Debug, Default)]
pub struct ApplicationManager {
    applications: RwLock<HashMap<Uuid, Arc<Application>>>,
}

impl ApplicationManager {
    pub async fn add(&self, app: Application) -> Option<Arc<Application>> {
        let app_id = app.config.properties.id;
        let thread_safe_app = app.into();

        self.applications
            .write()
            .await
            .insert(app_id, thread_safe_app)
    }

    pub async fn remove(&self, app_id: &Uuid) -> Option<Arc<Application>> {
        self.applications.write().await.remove(app_id)
    }
}

#[instrument]
pub async fn execute_new(
    manager: &Arc<ApplicationManager>,
    app_settings: AppCreationSettings,
) -> anyhow::Result<()> {
    debug!("starting application");

    let app_id = app_settings.properties.id;
    let app = Application::from(app_settings);
    app.subscribe_async_handler(Arc::new(ManagerHandler {
        manager: manager.clone(),
    }))
    .await;
    app.subscribe_async_handler(Arc::new(LogHandler::new(&app).await?))
        .await;

    manager.add(app).await;

    let apps = manager.applications.read().await;
    if let Some(app) = apps.get(&app_id) {
        app.start(app).await?;
    }

    Ok(())
}
