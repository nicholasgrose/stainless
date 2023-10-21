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

#[async_trait]
pub trait AppEventHandler: Send + Sync + Debug {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()>;
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
