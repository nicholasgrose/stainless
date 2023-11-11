use std::fmt::Debug;
use std::process::ExitStatus;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::{instrument, warn};

use crate::manager::app::Application;

pub mod asynchronous;
pub mod synchronous;

#[derive(Debug, Clone)]
pub struct AppEvent {
    pub application: Arc<Application>,
    pub event_type: AppEventType,
}

#[derive(Debug, Clone)]
pub enum AppEventType {
    Start {},
    End {
        result: Arc<anyhow::Result<ExitStatus>>,
    },
    Print {
        line: LineType,
    },
}

#[derive(Debug, Clone)]
pub enum LineType {
    Out(String),
    Error(String),
}

#[derive(Debug)]
pub enum AppEventHandler {
    Synchronous(Arc<dyn SyncAppEventHandler>),
    Asynchronous(Arc<dyn AsyncAppEventHandler>),
}

#[async_trait]
pub trait AsyncAppEventHandler: Send + Sync + Debug {
    async fn handle_async(&self, event: Arc<AppEvent>) -> anyhow::Result<()>;
}

#[async_trait]
pub trait SyncAppEventHandler: Send + Sync + Debug {
    async fn handle_sync(&self, event: Arc<AppEvent>) -> anyhow::Result<()>;
}

pub async fn send_event(app: &Arc<Application>, event: AppEventType) -> anyhow::Result<()> {
    let event = Arc::new(AppEvent {
        application: app.clone(),
        event_type: event,
    });

    send_to_handlers(app, Some(event)).await
}

pub async fn close_event_stream(app: &Arc<Application>) -> anyhow::Result<()> {
    send_to_handlers(app, None).await
}

async fn send_to_handlers(
    app: &Arc<Application>,
    event: Option<Arc<AppEvent>>,
) -> anyhow::Result<()> {
    app.send_async_event(&event).await?;
    app.send_sync_event(&event).await;

    Ok(())
}

async fn dispatch_task(
    handler: AppEventHandler,
    event: Arc<AppEvent>,
    app_span: Arc<tracing::Span>,
) {
    let _app_enter = app_span.enter();

    dispatch(handler, event).await
}

// This function is only ever called within the event app's span, so including data from the event
// other than the type becomes unnecessarily bloated and repetitive in resultant traces.
#[instrument(skip(event), fields(?event.event_type))]
async fn dispatch(handler: AppEventHandler, event: Arc<AppEvent>) {
    match handler {
        AppEventHandler::Synchronous(handler) => {
            process_handler_result(handler.handle_sync(event).await)
        }
        AppEventHandler::Asynchronous(handler) => {
            process_handler_result(handler.handle_async(event).await)
        }
    }
}

fn process_handler_result(result: anyhow::Result<()>) {
    match result {
        Ok(_) => {}
        Err(error) => {
            warn!(?error, "event handling failed")
        }
    }
}
