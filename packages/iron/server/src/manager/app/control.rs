use std::process::{ExitStatus, Stdio};
use std::sync::Arc;

use anyhow::Context;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::io::{AsyncRead, BufWriter};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::warn;

use crate::manager::app::events::{send_event, AppEventType, LineType};
use crate::manager::app::{AppRunState, Application};

const INPUT_BUFFER_SIZE: usize = 100;

macro_rules! safely {
    ($broadcast:expr) => {
        if let Err(e) = $broadcast.await {
            return Arc::new(Err(e));
        }
    };
}

impl Application {
    pub async fn start(&self, app: &Arc<Application>) -> anyhow::Result<()> {
        let mut state = self.state.write().await;

        match state.run_state {
            AppRunState::NotStarted => {
                self.initialize_app().await?;
                state.run_state = self.start_new_process(app.clone()).await?;
            }
            AppRunState::Running { .. } => {}
            AppRunState::Stopped => {
                state.run_state = self.start_new_process(app.clone()).await?;
            }
        }

        Ok(())
    }

    pub async fn initialize_app(&self) -> anyhow::Result<()> {
        tokio::fs::create_dir_all(&self.config.directory).await?;
        self.spawn_starting_listeners().await;

        Ok(())
    }

    async fn spawn_starting_listeners(&self) {
        for handler in &self.events.handlers.read().await.async_handlers {
            self.spawn_event_listener(handler.clone());
        }
    }

    pub async fn start_new_process(&self, app: Arc<Application>) -> anyhow::Result<AppRunState> {
        let (sender, receiver) = tokio::sync::mpsc::channel(INPUT_BUFFER_SIZE);
        let app_task = self.spawn_app_tasks(app, receiver).await?;

        Ok(AppRunState::Running {
            app_task,
            input_sender: sender,
        })
    }

    async fn spawn_app_tasks(
        &self,
        app: Arc<Application>,
        input_receiver: mpsc::Receiver<String>,
    ) -> anyhow::Result<JoinHandle<Arc<anyhow::Result<ExitStatus>>>> {
        let mut app_process = self.execute().await?;
        self.spawn_io_handlers(&mut app_process, &app, input_receiver)?;

        Ok(self.spawn_process_task(app, app_process))
    }

    async fn execute(&self) -> anyhow::Result<Child> {
        let command_args: Vec<&str> = self.config.properties.command.split(' ').collect();

        Ok(Command::new(command_args[0])
            .args(&command_args[1..])
            .current_dir(&self.config.directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?)
    }

    fn spawn_io_handlers(
        &self,
        process: &mut Child,
        app: &Arc<Application>,
        input_receiver: mpsc::Receiver<String>,
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
        self.spawn_output_handler(stdio_out, app.clone(), LineType::Out);
        self.spawn_output_handler(stdio_err, app.clone(), LineType::Error);

        Ok(())
    }

    fn spawn_input_handler(
        &self,
        child_in: ChildStdin,
        mut input_receiver: mpsc::Receiver<String>,
    ) {
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
        app_lock: Arc<Application>,
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
        app: Arc<Application>,
        mut app_process: Child,
    ) -> JoinHandle<Arc<anyhow::Result<ExitStatus>>> {
        let _enter = self.config.span.enter();
        let app_span = self.config.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            safely!(send_event(&app, AppEventType::Start {}));

            let execution_result = Arc::new(
                app_process
                    .wait()
                    .await
                    .context("error occurred while running application"),
            );

            app.state.write().await.run_state = AppRunState::Stopped;

            safely!(send_event(
                &app,
                AppEventType::End {
                    result: execution_result.clone(),
                }
            ));

            execution_result
        })
    }
}
