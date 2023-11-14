use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use std::path::PathBuf;
use std::process::ExitStatus;
use std::sync::Arc;

use crate::database::insert::{Insert, InsertModel};
use entity::app_args::ActiveModel as AppArgModel;
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
    pub program: AppArg,
    pub args: Vec<AppArg>,
}

#[derive(Debug)]
pub struct AppArg {
    pub id: Uuid,
    pub argument: String,
}

#[async_trait]
impl Insert<AppProperties> for AppCommand {
    async fn insert(
        &self,
        connection: &impl ConnectionTrait,
        context: &AppProperties,
    ) -> anyhow::Result<()> {
        let all_args = std::iter::once(&self.program)
            .chain(&self.args)
            .collect::<Vec<_>>();

        // We should iterate in reverse so that DB constraints don't hate us
        for arg_index in (0..all_args.len()).rev() {
            let arg = all_args[arg_index];

            arg.build_model(&AppArgModelContext {
                app_properties: context,
                next_arg: all_args.get(arg_index + 1).copied(),
            })
            .await?
            .insert(connection)
            .await?;
        }

        Ok(())
    }
}

impl AppCommand {
    fn executable(&self) -> Command {
        let mut command = Command::new(&self.program.argument);

        command.args(self.args.iter().map(|a| a.argument.as_str()));

        command
    }
}

impl<S> From<S> for AppArg
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        AppArg {
            id: Uuid::new_v4(),
            argument: value.into(),
        }
    }
}

#[derive(Debug)]
struct AppArgModelContext<'a> {
    app_properties: &'a AppProperties,
    next_arg: Option<&'a AppArg>,
}

#[async_trait]
impl<'a> InsertModel<AppArgModel, AppArgModelContext<'a>> for AppArg {
    async fn build_model(&self, context: &AppArgModelContext<'a>) -> anyhow::Result<AppArgModel> {
        Ok(AppArgModel {
            id: Set(self.id.to_string()),
            app_id: Set(context.app_properties.id.to_string()),
            argument: Set(self.argument.clone()),
            next_arg: Set(context.next_arg.map(|a| a.id.to_string())),
        })
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
