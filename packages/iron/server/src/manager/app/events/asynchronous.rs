use std::sync::Arc;

use anyhow::Context;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing::instrument;

use crate::manager::app::events::AppEventHandler;
use crate::manager::app::events::{dispatch_task, AppEvent};
use crate::manager::app::Application;

impl Application {
    #[instrument]
    pub async fn send_async_event(&self, event: &Arc<AppEvent>) -> anyhow::Result<()> {
        self.events
            .read()
            .await
            .async_channel
            .send(event.clone())
            .context("failed to broadcast app event")?;

        Ok(())
    }

    pub async fn subscribe_async_handler(
        &self,
        handler: Arc<dyn AppEventHandler>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let receiver = self.events.write().await.async_channel.subscribe();

        self.spawn_event_listener(receiver, handler)
    }

    pub fn spawn_event_listener(
        &self,
        mut receiver: broadcast::Receiver<Arc<AppEvent>>,
        handler: Arc<dyn AppEventHandler>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let _enter = self.config.span.enter();
        let app_span = self.config.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            loop {
                let event = receiver.recv().await?;

                tokio::spawn(dispatch_task(
                    handler.clone(),
                    event.clone(),
                    app_span.clone(),
                ));
            }
        })
    }
}
