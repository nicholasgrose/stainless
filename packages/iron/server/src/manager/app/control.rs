use std::process::ExitStatus;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use tokio::io::AsyncWriteExt;
use tokio::process::Child;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tokio::{pin, select};

use crate::manager::app::events::AppEvent;
use crate::manager::app::{Application, ApplicationState};

pub async fn start(app_lock: &Arc<RwLock<Application>>) -> anyhow::Result<()> {
    let mut app = app_lock.write().await;

    let (sender, receiver) = tokio::sync::mpsc::channel(100);
    let app_process = app.execute().await?;
    let app_task = create_app_process_task(app_lock, app_process, receiver);

    app.state = ApplicationState::Active {
        app_task,
        input_sender: sender,
    };

    Ok(())
}

fn create_app_process_task(
    app_lock: &Arc<RwLock<Application>>,
    app_process: Child,
    receiver: mpsc::Receiver<u8>,
) -> JoinHandle<Arc<anyhow::Result<ExitStatus>>> {
    let task_lock = app_lock.clone();

    tokio::spawn(async move {
        let result = broadcast_event(
            &task_lock,
            AppEvent::Start {
                application: task_lock.clone(),
            },
        )
        .await;

        if let Err(e) = result {
            return Arc::new(Err(e));
        }

        let execution_result = Arc::new(attach_receiver_to_process(receiver, app_process).await);

        let result = broadcast_event(
            &task_lock,
            AppEvent::End {
                application: task_lock.clone(),
                result: execution_result.clone(),
            },
        )
        .await;

        if let Err(e) = result {
            return Arc::new(Err(e));
        }

        execution_result
    })
}

async fn broadcast_event(
    app_lock: &Arc<RwLock<Application>>,
    event: AppEvent,
) -> anyhow::Result<usize> {
    app_lock
        .read()
        .await
        .events
        .send(event)
        .context("failed to broadcast app process event")
}

pub async fn attach_receiver_to_process(
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
                return application_result.context("error occurred running application")
            }
        }
    }
}
