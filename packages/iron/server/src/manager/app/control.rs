use std::process::ExitStatus;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use tokio::io::AsyncWriteExt;
use tokio::process::Child;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tokio::{pin, select};
use tracing::instrument;

use crate::manager::app::events::AppEvent;
use crate::manager::app::{Application, ApplicationState, EventListenerCommand};

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
        let app_task = self.create_app_process_task(app_lock, app_process, receiver);

        self.state = ApplicationState::Active {
            app_task,
            input_sender: sender,
        };

        Ok(())
    }

    fn create_app_process_task(
        &self,
        app_lock: Arc<RwLock<Application>>,
        app_process: Child,
        receiver: mpsc::Receiver<u8>,
    ) -> JoinHandle<Arc<anyhow::Result<ExitStatus>>> {
        let app_span = self.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            safely!(trigger_event(
                &app_lock,
                AppEvent::Start {
                    application: app_lock.clone(),
                }
            ));

            let execution_result =
                Arc::new(attach_receiver_to_process(receiver, app_process).await);

            safely!(trigger_event(
                &app_lock,
                AppEvent::End {
                    application: app_lock.clone(),
                    result: execution_result.clone(),
                }
            ));

            safely!(broadcast_final_event(&app_lock));

            execution_result
        })
    }

    #[instrument]
    async fn broadcast_command(
        &self,
        listener_command: EventListenerCommand,
    ) -> anyhow::Result<usize, Arc<anyhow::Result<ExitStatus>>> {
        self.events
            .send(listener_command)
            .context("failed to broadcast app process event")
            .map_err(|e| Arc::new(Err(e)))
    }
}

async fn trigger_event(
    app_lock: &Arc<RwLock<Application>>,
    event: AppEvent,
) -> anyhow::Result<(), Arc<anyhow::Result<ExitStatus>>> {
    let app = app_lock.read().await;
    let event = Arc::new(event);
    let listener_command = EventListenerCommand::Dispatch(event.clone());

    app.broadcast_command(listener_command).await?;
    app.send_sync_event(&event).await;

    Ok(())
}

async fn broadcast_final_event(
    app_lock: &Arc<RwLock<Application>>,
) -> anyhow::Result<usize, Arc<anyhow::Result<ExitStatus>>> {
    app_lock
        .read()
        .await
        .broadcast_command(EventListenerCommand::Close)
        .await
}

pub async fn attach_receiver_to_process(
    mut receiver: mpsc::Receiver<u8>,
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
                return application_result.context("error occurred running application")
            }
        }
    }
}
