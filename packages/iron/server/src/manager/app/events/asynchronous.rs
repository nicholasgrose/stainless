use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::manager::app::events::dispatch_task;
use crate::manager::app::events::AppEventHandler;
use crate::manager::app::{Application, EventListenerCommand};

impl Application {
    pub fn subscribe_async_handler(
        &self,
        handler: Arc<dyn AppEventHandler>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let receiver = self.events.subscribe();

        self.spawn_event_listener(receiver, handler)
    }

    pub fn spawn_event_listener(
        &self,
        mut receiver: broadcast::Receiver<EventListenerCommand>,
        handler: Arc<dyn AppEventHandler>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let _enter = self.span.enter();
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
