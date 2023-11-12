use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::ExitStatus;
use std::sync::Arc;

use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::{broadcast, RwLock};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::manager::app::events::{AppEvent, AsyncAppEventHandler, SyncAppEventHandler};

pub mod control;
pub mod create;
pub mod events;

#[derive(Debug)]
pub struct Application {
    pub config: AppConfig,
    pub events: AppEvents,
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
    pub command: AppCommand,
}

#[derive(Debug)]
pub struct AppCommand {
    pub program: String,
    pub args: Vec<String>,
}

impl Display for AppCommand {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format!("{} {}", self.program, self.args.join(" ")))
    }
}

impl AppCommand {
    fn executable(&self) -> Command {
        let mut command = Command::new(&self.program);
        command.args(&self.args);

        command
    }
}

#[derive(Debug)]
pub struct AppState {
    pub run_state: AppRunState,
}

#[derive(Debug)]
pub enum AppRunState {
    NotStarted,
    Running {
        app_task: JoinHandle<Arc<anyhow::Result<ExitStatus>>>,
        input_sender: mpsc::Sender<String>,
    },
    Stopped {
        result: Arc<anyhow::Result<ExitStatus>>,
    },
}

#[derive(Debug)]
pub struct AppEvents {
    async_channel: broadcast::Sender<Option<Arc<AppEvent>>>,
    pub handlers: RwLock<AppEventHandlers>,
}

#[derive(Debug)]
pub struct AppEventHandlers {
    pub async_handlers: Vec<Arc<dyn AsyncAppEventHandler>>,
    pub sync_handlers: Vec<Arc<dyn SyncAppEventHandler>>,
}
