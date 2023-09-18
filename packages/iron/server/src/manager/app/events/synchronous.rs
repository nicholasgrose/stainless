use std::sync::Arc;

use tokio::task::JoinSet;

use crate::manager::app::events::dispatch_task;
use crate::manager::app::events::{AppEvent, AppEventHandler};
use crate::manager::app::Application;

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
}
