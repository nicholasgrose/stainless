use std::sync::Arc;

use anyhow::Context;
use tracing::instrument;

use crate::manager::app::events::AppEvent;
use crate::manager::app::Application;

impl Application {
    #[instrument]
    pub async fn send_to_listeners(&self, event: &Arc<AppEvent>) -> anyhow::Result<()> {
        self.events
            .send(event.clone())
            .context("failed to broadcast app process event")?;

        Ok(())
    }
}
