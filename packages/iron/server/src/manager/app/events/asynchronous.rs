use std::sync::Arc;

use anyhow::Context;
use tokio::task::JoinHandle;
use tracing::instrument;

use crate::manager::app::events::{dispatch_task, AppEvent};
use crate::manager::app::events::{AppEventHandler, AsyncAppEventHandler};
use crate::manager::app::{AppRunState, Application};

impl Application {
    #[instrument]
    pub async fn send_async_event(&self, event: &Option<Arc<AppEvent>>) -> anyhow::Result<()> {
        self.events
            .async_channel
            .send(event.clone())
            .context("failed to broadcast app event")?;

        Ok(())
    }

    pub async fn subscribe_async_handler(&self, handler: Arc<dyn AsyncAppEventHandler>) {
        self.events
            .handlers
            .write()
            .await
            .async_handlers
            .push(handler.clone());

        match self.state.read().await.run_state {
            AppRunState::NotStarted => {}
            AppRunState::Running { .. } => {
                self.spawn_event_listener(handler);
            }
            AppRunState::Stopped { .. } => {
                self.spawn_event_listener(handler);
            }
        }
    }

    pub fn spawn_event_listener(
        &self,
        handler: Arc<dyn AsyncAppEventHandler>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let mut receiver = self.events.async_channel.subscribe();

        let _enter = self.config.span.enter();
        let app_span = self.config.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            loop {
                let next_event = receiver.recv().await?;

                match next_event {
                    None => {
                        return Ok(());
                    }
                    Some(event) => {
                        tokio::spawn(dispatch_task(
                            AppEventHandler::Asynchronous(handler.clone()),
                            event.clone(),
                            app_span.clone(),
                        ));
                    }
                }
            }
        })
    }
}
