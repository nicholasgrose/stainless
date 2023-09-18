use std::process::ExitStatus;
use std::sync::Arc;

use anyhow::Context;
use tokio::sync::RwLock;
use tracing::instrument;

use crate::manager::app::{Application, EventListenerCommand};

impl Application {
    #[instrument]
    pub async fn send_to_listeners(
        &self,
        listener_command: EventListenerCommand,
    ) -> anyhow::Result<(), Arc<anyhow::Result<ExitStatus>>> {
        self.events
            .send(listener_command)
            .context("failed to broadcast app process event")
            .map_err(|e| Arc::new(Err(e)))?;

        Ok(())
    }
}

pub async fn stop_command_listeners(
    app_lock: &Arc<RwLock<Application>>,
) -> anyhow::Result<(), Arc<anyhow::Result<ExitStatus>>> {
    app_lock
        .read()
        .await
        .send_to_listeners(EventListenerCommand::Close)
        .await
}
