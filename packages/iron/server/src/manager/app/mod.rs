use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};

use anyhow::anyhow;
use anyhow::Context;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::{pin, select};
use tracing::{info, span, Level};
use uuid::Uuid;

use crate::manager::app::events::{AppEvent, AppEventDispatcher};

pub mod events;

#[derive(Debug)]
pub struct Application {
    pub settings: ApplicationSettings,
    pub state: ApplicationState,
}

#[derive(Debug)]
pub struct ApplicationSettings {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    pub event_handlers: Vec<Box<dyn AppEventDispatcher>>,
}

#[derive(Debug)]
pub enum ApplicationState {
    Inactive,
    Active {
        app_task: JoinHandle<anyhow::Result<ExitStatus>>,
        input_sender: Sender<u8>,
    },
}

impl Application {
    pub(crate) fn new(settings: ApplicationSettings) -> Self {
        Application {
            settings,
            state: ApplicationState::Inactive,
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let app_process = self.execute().await?;
        let app_task = self.create_process_control_task(app_process, receiver);

        self.state = ApplicationState::Active {
            app_task,
            input_sender: sender,
        };

        Ok(())
    }

    async fn execute(&self) -> anyhow::Result<Child> {
        let working_directory = self.working_directory().await?;
        let log_file = File::create(working_directory.join("application.log")).await?;
        let command_args: Vec<&str> = self.settings.command.split(' ').collect();

        Ok(Command::new(command_args[0])
            .args(&command_args[1..])
            .current_dir(&working_directory)
            .stdin(Stdio::piped())
            .stdout(log_file.into_std().await)
            .stderr(Stdio::null())
            .spawn()?)
    }

    async fn working_directory(&self) -> anyhow::Result<PathBuf> {
        let settings = &self.settings;
        let path = format!("{}_{}", settings.name, settings.id).into();
        tokio::fs::create_dir_all(&path).await?;

        Ok(path)
    }

    fn create_process_control_task(
        &self,
        app_process: Child,
        receiver: Receiver<u8>,
    ) -> JoinHandle<anyhow::Result<ExitStatus>> {
        let id = self.settings.id.to_string();

        tokio::spawn(async move {
            let span = span!(Level::INFO, "app task", "uuid" = id);
            let _enter = span.enter();
            info!("starting task");

            let execution_result = attach_receiver_to_process(receiver, app_process).await;
            let _enter = span.enter();

            match &execution_result {
                Ok(status) => {
                    info!("task exited with {}", status)
                }
                Err(error) => {
                    info!("task failed with {}", error)
                }
            }

            execution_result
        })
    }
}

async fn attach_receiver_to_process(
    mut receiver: Receiver<u8>,
    mut process: Child,
) -> anyhow::Result<ExitStatus> {
    let mut child_in = process
        .stdin
        .take()
        .context("new application process lacks stdin")?;
    let application_task = process.wait();
    pin!(application_task);

    loop {
        select! {
            receive_result = receiver.recv() => {
                match receive_result {
                    Some(byte) => child_in.write_u8(byte).await?,
                    None => return Err(anyhow!("input channel broke"))
                }
            }
            application_result = &mut application_task => {
                return application_result.with_context(|| "error occurred running application")
            }
        }
    }
}
