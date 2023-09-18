use std::path::PathBuf;
use std::process::ExitStatus;
use std::sync::Arc;

use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::info_span;
use uuid::Uuid;

use crate::manager::app::events::{AppEvent, AppEventHandler};
use crate::manager::log_handler::LogHandler;

pub mod control;
pub mod events;

#[derive(Debug)]
pub struct AppCreationSettings {
    pub properties: AppProperties,
    pub async_event_handlers: Vec<Arc<dyn AppEventHandler>>,
    pub sync_event_handlers: Vec<Arc<dyn AppEventHandler>>,
}

#[derive(Debug)]
pub struct Application {
    pub span: Arc<tracing::Span>,
    pub properties: AppProperties,
    pub state: ApplicationState,
    pub events: broadcast::Sender<EventListenerCommand>,
    pub sync_event_handlers: Vec<Arc<dyn AppEventHandler>>,
}

#[derive(Debug)]
pub struct AppProperties {
    pub id: Uuid,
    pub name: String,
    pub command: String,
}

#[derive(Debug)]
pub enum ApplicationState {
    Inactive,
    Active {
        app_task: JoinHandle<Arc<anyhow::Result<ExitStatus>>>,
        input_sender: mpsc::Sender<u8>,
    },
}

#[derive(Clone, Debug)]
pub enum EventListenerCommand {
    Dispatch(Arc<AppEvent>),
    Close,
}

impl Application {
    pub fn new(settings: AppCreationSettings) -> Self {
        let (sender, receiver) = broadcast::channel(4);
        let app = Application {
            span: Arc::new(info_span!(parent: None, "app", ?settings.properties)),
            properties: settings.properties,
            state: ApplicationState::Inactive,
            events: sender,
            sync_event_handlers: settings.sync_event_handlers,
        };

        {
            let _enter = app.span.enter();

            app.spawn_event_listener(receiver, Arc::<LogHandler>::default());

            for handler in settings.async_event_handlers {
                app.subscribe_async_handler(handler);
            }
        }

        app
    }

    async fn working_directory(&self) -> anyhow::Result<PathBuf> {
        let settings = &self.properties;
        let path = format!("{}_{}", settings.name, settings.id).into();
        tokio::fs::create_dir_all(&path).await?;

        Ok(path)
    }
}
