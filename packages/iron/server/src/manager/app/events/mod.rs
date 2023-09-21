use std::fmt::Debug;
use std::process::ExitStatus;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{instrument, warn};

use crate::manager::app::Application;

pub mod asynchronous;
pub mod listener;
pub mod synchronous;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Start {
        application: Arc<RwLock<Application>>,
    },
    End {
        application: Arc<RwLock<Application>>,
        result: Arc<anyhow::Result<ExitStatus>>,
    },
    LineOut {
        application: Arc<RwLock<Application>>,
        line: String,
    },
    ErrorLineOut {
        application: Arc<RwLock<Application>>,
        line: String,
    },
}

#[async_trait]
pub trait AppEventHandler: Send + Sync + Debug {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()>;
}

pub async fn send_event(
    app_lock: &Arc<RwLock<Application>>,
    event: AppEvent,
) -> anyhow::Result<()> {
    let event = Arc::new(event);
    let app = app_lock.read().await;

    app.send_to_listeners(&event).await?;
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
