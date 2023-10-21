use std::sync::Arc;

use tokio::task::JoinSet;

use crate::manager::app::events::dispatch_task;
use crate::manager::app::events::{AppEvent, AppEventHandler};
use crate::manager::app::Application;

impl Application {
    pub async fn _subscribe_sync_handler(&self, handler: Arc<dyn AppEventHandler>) {
        self.events
            .handlers
            .write()
            .await
            .sync_handlers
            .push(handler);
    }

    pub async fn send_sync_event(&self, event: &Option<Arc<AppEvent>>) {
        if let Some(event) = event {
            let mut handler_pool = self.dispatch_sync_events(event).await;
            self.await_dispatch_results(&mut handler_pool).await;
        }
    }

    async fn dispatch_sync_events(&self, event: &Arc<AppEvent>) -> JoinSet<()> {
        let mut handler_pool = JoinSet::new();
        let sync_handlers = &self.events.handlers.read().await.sync_handlers;

        for handler in sync_handlers {
            let app_span = self.config.span.clone();
            let dispatch_handler = handler.clone();
            let dispatch_event = event.clone();

            handler_pool.spawn(dispatch_task(dispatch_handler, dispatch_event, app_span));
        }

        handler_pool
    }

    async fn await_dispatch_results(&self, handler_pool: &mut JoinSet<()>) {
        loop {
            match handler_pool.join_next().await {
                None => {
                    break;
                }
                Some(_result) => {}
            }
        }
    }
}
