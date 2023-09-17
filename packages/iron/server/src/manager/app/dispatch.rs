use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing::{instrument, warn};

use crate::manager::app::control::EventReceiverCommand;
use crate::manager::app::events::{AppEvent, AppEventDispatcher};
use crate::manager::app::Application;

impl Application {
    pub fn subscribe_dispatcher(
        &self,
        dispatcher: Arc<dyn AppEventDispatcher>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let receiver = self.events.subscribe();

        self.attach_receiver_to_dispatcher(receiver, dispatcher)
    }

    pub fn attach_receiver_to_dispatcher(
        &self,
        mut receiver: broadcast::Receiver<EventReceiverCommand>,
        dispatcher: Arc<dyn AppEventDispatcher>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let app_span = self.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            loop {
                let app_event = receiver.recv().await?;

                match app_event {
                    EventReceiverCommand::Close => {
                        return Ok(());
                    }
                    EventReceiverCommand::Dispatch(event) => {
                        spawn_dispatch_async(dispatcher.clone(), event.clone(), app_span.clone());
                        dispatch_sync(dispatcher.clone(), event);
                    }
                }
            }
        })
    }
}

fn spawn_dispatch_async(
    dispatcher: Arc<dyn AppEventDispatcher>,
    event: Arc<AppEvent>,
    app_span: Arc<tracing::Span>,
) {
    tokio::spawn(async move {
        let _app_enter = app_span.enter();

        dispatch_async(dispatcher, event).await
    });
}

#[instrument]
async fn dispatch_async(dispatcher: Arc<dyn AppEventDispatcher>, event: Arc<AppEvent>) {
    match dispatcher.dispatch(event.clone()).await {
        Ok(_) => {}
        Err(error) => {
            warn!(?event, ?error)
        }
    }
}

#[instrument]
fn dispatch_sync(dispatcher: Arc<dyn AppEventDispatcher>, event: Arc<AppEvent>) {
    match dispatcher.dispatch_sync(event.clone()) {
        Ok(_) => {}
        Err(error) => {
            warn!(?event, ?error)
        }
    }
}
