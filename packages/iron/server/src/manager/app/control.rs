use std::process::{ExitStatus, Stdio};
use std::sync::Arc;

use anyhow::Context;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::io::{AsyncRead, BufWriter};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tracing::warn;

use crate::manager::app::events::{send_event, AppEventType, LineType};
use crate::manager::app::{Application, ApplicationState};

const INPUT_BUFFER_SIZE: usize = 100;

macro_rules! safely {
    ($broadcast:expr) => {
        if let Err(e) = $broadcast.await {
            return Arc::new(Err(e));
        }
    };
}

impl Application {
    pub async fn start(&mut self, app_lock: Arc<RwLock<Application>>) -> anyhow::Result<()> {
        let (sender, receiver) = tokio::sync::mpsc::channel(INPUT_BUFFER_SIZE);
        let app_task = self.spawn_app_tasks(app_lock, receiver).await?;

        self.state = ApplicationState::Active {
            app_task,
            input_sender: sender,
        };

        Ok(())
    }

    async fn spawn_app_tasks(
        &self,
        app_lock: Arc<RwLock<Application>>,
        input_receiver: mpsc::Receiver<String>,
    ) -> anyhow::Result<JoinHandle<Arc<anyhow::Result<ExitStatus>>>> {
        let mut app_process = self.execute().await?;
        self.spawn_io_handlers(&mut app_process, &app_lock, input_receiver)?;

        Ok(self.spawn_process_task(app_lock, app_process))
    }

    async fn execute(&self) -> anyhow::Result<Child> {
        let working_directory = self.working_directory().await?;
        let command_args: Vec<&str> = self.properties.command.split(' ').collect();

        Ok(Command::new(command_args[0])
            .args(&command_args[1..])
            .current_dir(&working_directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?)
    }

    fn spawn_io_handlers(
        &self,
        process: &mut Child,
        app_lock: &Arc<RwLock<Application>>,
        input_receiver: Receiver<String>,
    ) -> anyhow::Result<()> {
        let stdio_in = process
            .stdin
            .take()
            .context("new application process lacks stdin")?;
        let stdio_out = process
            .stdout
            .take()
            .context("new application process lacks stdout")?;
        let stdio_err = process
            .stderr
            .take()
            .context("new application process lacks stderr")?;

        self.spawn_input_handler(stdio_in, input_receiver);
        self.spawn_output_handler(stdio_out, app_lock.clone(), |line| LineType::Out(line));
        self.spawn_output_handler(stdio_err, app_lock.clone(), |line| LineType::Error(line));

        Ok(())
    }

    fn spawn_input_handler(&self, child_in: ChildStdin, mut input_receiver: Receiver<String>) {
        tokio::spawn(async move {
            let mut input_writer = BufWriter::new(child_in);

            loop {
                let receive_result = input_receiver.recv().await;

                match receive_result {
                    Some(bytes) => match input_writer.write_all(bytes.as_bytes()).await {
                        Ok(_) => {}
                        Err(error) => {
                            warn!(?error)
                        }
                    },
                    None => return anyhow::Result::<()>::Ok(()),
                }
            }
        });
    }

    fn spawn_output_handler<T>(
        &self,
        child_out: T,
        app_lock: Arc<RwLock<Application>>,
        event_provider: fn(String) -> LineType,
    ) where
        T: AsyncRead + Unpin + Send + 'static,
    {
        tokio::spawn(async move {
            let mut output_reader = BufReader::new(child_out);
            let mut line = String::new();

            loop {
                let bytes_read = output_reader.read_line(&mut line).await;

                match bytes_read {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            return anyhow::Result::<()>::Ok(());
                        }

                        send_event(
                            &app_lock,
                            AppEventType::Print {
                                line: event_provider(line.clone()),
                            },
                        )
                        .await?;
                    }
                    Err(error) => {
                        warn!(?error);
                    }
                }
            }
        });
    }

    fn spawn_process_task(
        &self,
        app_lock: Arc<RwLock<Application>>,
        mut app_process: Child,
    ) -> JoinHandle<Arc<anyhow::Result<ExitStatus>>> {
        let _enter = self.span.enter();
        let app_span = self.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            safely!(send_event(&app_lock, AppEventType::Start {}));

            let execution_result = Arc::new(
                app_process
                    .wait()
                    .await
                    .context("error occurred while running application"),
            );

            app_lock.write().await.state = ApplicationState::Inactive;

            safely!(send_event(
                &app_lock,
                AppEventType::End {
                    result: execution_result.clone(),
                }
            ));

            execution_result
        })
    }
}
