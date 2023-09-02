use std::collections::HashMap;
use std::fmt::Debug;

use tokio::sync::RwLock;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::manager::app::{Application, ApplicationSettings};

pub mod app;

#[derive(Debug, Default)]
pub struct ApplicationManager {
    applications: RwLock<HashMap<Uuid, Application>>,
}

impl ApplicationManager {
    #[instrument(skip(self))]
    pub async fn execute_new(&self, app_settings: ApplicationSettings) -> anyhow::Result<()> {
        info!("starting application");

        let app_id = app_settings.id;
        let app = Application::new(app_settings);

        let mut applications = self.applications.write().await;

        applications.insert(app_id, app);
        applications.get_mut(&app_id).map(|a| a.start());

        Ok(())
    }
}
