use std::path::PathBuf;
use std::process::ExitStatus;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::sync::{broadcast, RwLock};
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
    pub config: AppConfig,
    pub events: RwLock<AppEvents>,
    pub state: RwLock<AppState>,
}

#[derive(Debug)]
pub struct AppConfig {
    pub properties: AppProperties,
    pub span: Arc<tracing::Span>,
    pub directory: PathBuf,
}

#[derive(Debug)]
pub struct AppProperties {
    pub id: Uuid,
    pub name: String,
    pub command: String,
}

#[derive(Debug)]
pub struct AppState {
    run_state: AppRunState,
}

#[derive(Debug)]
pub enum AppRunState {
    Inactive,
    Active {
        app_task: JoinHandle<Arc<anyhow::Result<ExitStatus>>>,
        input_sender: mpsc::Sender<String>,
    },
}

#[derive(Debug)]
pub struct AppEvents {
    pub async_channel: broadcast::Sender<Arc<AppEvent>>,
    pub sync_handlers: Vec<Arc<dyn AppEventHandler>>,
}

const EVENT_CHANNEL_SIZE: usize = 16;

impl Application {
    pub async fn new(settings: AppCreationSettings) -> anyhow::Result<Self> {
        let (sender, receiver) = broadcast::channel(EVENT_CHANNEL_SIZE);
        let app = Application {
            config: AppConfig {
                span: Arc::new(info_span!(parent: None, "app", ?settings.properties)),
                directory: format!("{}_{}", settings.properties.name, settings.properties.id)
                    .into(),
                properties: settings.properties,
            },
            events: AppEvents {
                async_channel: sender,
                sync_handlers: settings.sync_event_handlers,
            }
            .into(),
            state: AppState {
                run_state: AppRunState::Inactive,
            }
            .into(),
        };

        tokio::fs::create_dir_all(&app.config.directory).await?;
        app.spawn_starting_listeners(settings.async_event_handlers, receiver)
            .await?;

        Ok(app)
    }

    async fn spawn_starting_listeners(
        &self,
        starting_listeners: Vec<Arc<dyn AppEventHandler>>,
        event_receiver: broadcast::Receiver<Arc<AppEvent>>,
    ) -> anyhow::Result<()> {
        let log_handler = LogHandler::new(self).await?;
        self.spawn_event_listener(event_receiver, Arc::new(log_handler));

        for handler in starting_listeners {
            self.subscribe_async_handler(handler).await;
        }

        Ok(())
    }
}
