use std::process::{ExitStatus, Stdio};
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tokio::{pin, select};

use crate::manager::app::events::AppEvent;
use crate::manager::app::events::{listener::stop_command_listeners, send_event};
use crate::manager::app::{Application, ApplicationState};

macro_rules! safely {
    ($broadcast:expr) => {
        if let Err(e) = $broadcast.await {
            return e;
        }
    };
}

impl Application {
    pub async fn start(&mut self, app_lock: Arc<RwLock<Application>>) -> anyhow::Result<()> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let app_process = self.execute().await?;
        let app_task = self.spawn_app_task(app_lock, app_process, receiver);

        self.state = ApplicationState::Active {
            app_task,
            input_sender: sender,
        };

        Ok(())
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

    fn spawn_app_task(
        &self,
        app_lock: Arc<RwLock<Application>>,
        app_process: Child,
        receiver: mpsc::Receiver<u8>,
    ) -> JoinHandle<Arc<anyhow::Result<ExitStatus>>> {
        let app_span = self.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            safely!(send_event(
                &app_lock,
                AppEvent::Start {
                    application: app_lock.clone(),
                }
            ));

            let execution_result =
                Arc::new(process_with_input_receiver(app_process, receiver).await);

            safely!(send_event(
                &app_lock,
                AppEvent::End {
                    application: app_lock.clone(),
                    result: execution_result.clone(),
                }
            ));

            safely!(stop_command_listeners(&app_lock));

            execution_result
        })
    }
}

async fn process_with_input_receiver(
    mut process: Child,
    mut receiver: mpsc::Receiver<u8>,
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
                return application_result.context("error occurred running application")
            }
        }
    }
}
