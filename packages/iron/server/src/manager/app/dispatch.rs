use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::task::{JoinHandle, JoinSet};
use tracing::{instrument, warn};

use crate::manager::app::events::{AppEvent, AppEventHandler};
use crate::manager::app::{Application, EventListenerCommand};

impl Application {
    pub fn _subscribe_sync_handler(&mut self, handler: Arc<dyn AppEventHandler>) {
        self.sync_event_handlers.push(handler);
    }

    pub async fn send_sync_event(&self, event: &Arc<AppEvent>) {
        let mut handler_pool = JoinSet::new();

        self.dispatch_sync_events(&mut handler_pool, event);
        self.await_sync_handler_results(&mut handler_pool).await;
    }

    fn dispatch_sync_events(&self, handler_pool: &mut JoinSet<()>, event: &Arc<AppEvent>) {
        for handler in &self.sync_event_handlers {
            let app_span = self.span.clone();
            let dispatch_handler = handler.clone();
            let dispatch_event = event.clone();

            handler_pool.spawn(dispatch_task(dispatch_handler, dispatch_event, app_span));
        }
    }

    async fn await_sync_handler_results(&self, handler_pool: &mut JoinSet<()>) {
        loop {
            match handler_pool.join_next().await {
                None => {
                    break;
                }
                Some(_result) => {}
            }
        }
    }

    pub fn subscribe_async_handler(
        &self,
        handler: Arc<dyn AppEventHandler>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let receiver = self.events.subscribe();

        self.spawn_listener_for_handler(receiver, handler)
    }

    pub fn spawn_listener_for_handler(
        &self,
        mut receiver: broadcast::Receiver<EventListenerCommand>,
        handler: Arc<dyn AppEventHandler>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let app_span = self.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            loop {
                let app_event = receiver.recv().await?;

                match app_event {
                    EventListenerCommand::Close => {
                        return Ok(());
                    }
                    EventListenerCommand::Dispatch(event) => {
                        tokio::spawn(dispatch_task(
                            handler.clone(),
                            event.clone(),
                            app_span.clone(),
                        ));
                    }
                }
            }
        })
    }
}

async fn dispatch_task(
    handler: Arc<dyn AppEventHandler>,
    event: Arc<AppEvent>,
    app_span: Arc<tracing::Span>,
) {
    let _app_enter = app_span.enter();

    dispatch(handler, event).await
}

#[instrument]
async fn dispatch(handler: Arc<dyn AppEventHandler>, event: Arc<AppEvent>) {
    match handler.handle(event.clone()).await {
        Ok(_) => {}
        Err(error) => {
            warn!(?event, ?error)
        }
    }
}
