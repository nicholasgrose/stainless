use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use std::sync::Arc;

use tokio::fs::File;
use tokio::process::{Child, Command};
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::warn;
use uuid::Uuid;

use crate::manager::app::control::EventReceiverCommand;
use crate::manager::app::events::AppEventDispatcher;
use crate::manager::log_dispatcher::LogDispatcher;

pub mod control;
pub mod events;

#[derive(Debug)]
pub struct AppCreationSettings {
    pub properties: AppProperties,
    pub starting_handlers: Vec<Arc<dyn AppEventDispatcher>>,
}

#[derive(Debug)]
pub struct Application {
    pub properties: AppProperties,
    pub state: ApplicationState,
    pub events: broadcast::Sender<EventReceiverCommand>,
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

impl Application {
    pub fn new(settings: AppCreationSettings) -> Self {
        let (sender, receiver) = broadcast::channel(4);
        let app = Application {
            properties: settings.properties,
            state: ApplicationState::Inactive,
            events: sender,
        };

        attach_receiver_to_dispatcher(receiver, Arc::new(LogDispatcher::new(&app.properties.id)));

        for handler in settings.starting_handlers {
            app.subscribe_dispatcher(handler);
        }

        app
    }

    pub fn subscribe_dispatcher(
        &self,
        dispatcher: Arc<dyn AppEventDispatcher>,
    ) -> JoinHandle<anyhow::Result<()>> {
        let receiver = self.events.subscribe();

        attach_receiver_to_dispatcher(receiver, dispatcher)
    }

    async fn execute(&self) -> anyhow::Result<Child> {
        let working_directory = self.working_directory().await?;
        let log_file = File::create(working_directory.join("application.log")).await?;
        let command_args: Vec<&str> = self.properties.command.split(' ').collect();

        Ok(Command::new(command_args[0])
            .args(&command_args[1..])
            .current_dir(&working_directory)
            .stdin(Stdio::piped())
            .stdout(log_file.into_std().await)
            .stderr(Stdio::null())
            .spawn()?)
    }

    async fn working_directory(&self) -> anyhow::Result<PathBuf> {
        let settings = &self.properties;
        let path = format!("{}_{}", settings.name, settings.id).into();
        tokio::fs::create_dir_all(&path).await?;

        Ok(path)
    }
}

fn attach_receiver_to_dispatcher(
    mut receiver: broadcast::Receiver<EventReceiverCommand>,
    dispatcher: Arc<dyn AppEventDispatcher>,
) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        loop {
            let app_event = receiver.recv().await?;

            match app_event {
                EventReceiverCommand::Close => {
                    return Ok(());
                }
                EventReceiverCommand::Dispatch(event) => {
                    let task_dispatcher = dispatcher.clone();

                    tokio::spawn(async move {
                        match task_dispatcher.dispatch(event.clone()).await {
                            Ok(_) => {}
                            Err(error) => {
                                warn!(?event, ?error)
                            }
                        }
                    });
                }
            }
        }
    })
}
