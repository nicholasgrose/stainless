use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};

use anyhow::anyhow;
use anyhow::Context;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::{pin, select};
use tracing::{info, instrument, span, Level};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct ApplicationManager {
    active_servers: RwLock<HashMap<Uuid, ActiveApplication>>,
}

impl ApplicationManager {
    #[instrument]
    pub async fn start_application(&self, application: Application) -> anyhow::Result<()> {
        info!("starting application");

        let application_id = application.id;
        let server = application.start().await?;
        self.active_servers
            .write()
            .await
            .insert(application_id, server);

        Ok(())
    }
}

#[derive(Debug)]
struct ActiveApplication {
    application: Application,
    app_task: JoinHandle<anyhow::Result<ExitStatus>>,
    input_sender: Sender<u8>,
}

#[derive(Debug)]
pub struct Application {
    pub id: Uuid,
    pub name: String,
    pub command: String,
}

impl Application {
    async fn start(self) -> anyhow::Result<ActiveApplication> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let app_process = self.execute().await?;
        let app_task = tokio::spawn(async move {
            let span = span!(Level::INFO, "app task", "uuid" = self.id.to_string());
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
        });

        Ok(ActiveApplication {
            application: self,
            app_task,
            input_sender: sender,
        })
    }

    async fn execute(&self) -> anyhow::Result<Child> {
        let working_directory = self.working_directory();
        tokio::fs::create_dir_all(&working_directory).await?;

        let log_file = File::create(working_directory.join("application.log"))?;
        let out_io = Stdio::from(log_file);

        let command_args: Vec<&str> = self.command.split(' ').collect();

        Ok(Command::new(command_args[0])
            .args(&command_args[1..])
            .current_dir(self.working_directory())
            .stdin(Stdio::piped())
            .stdout(out_io)
            .stderr(Stdio::null())
            .spawn()?)
    }

    fn working_directory(&self) -> PathBuf {
        format!("{}_{}", self.name, self.id).into()
    }
}

async fn attach_receiver_to_process(
    mut receiver: Receiver<u8>,
    mut process: Child,
) -> anyhow::Result<ExitStatus> {
    let mut child_in = process
        .stdin
        .take()
        .with_context(|| "new application process lacks stdin")?;
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
