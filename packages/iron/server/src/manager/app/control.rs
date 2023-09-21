use std::process::{ExitStatus, Stdio};
use std::sync::Arc;

use anyhow::Context;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::io::{AsyncRead, BufWriter};
use tokio::process::{Child, Command};
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tracing::warn;

use crate::manager::app::events::send_event;
use crate::manager::app::events::AppEvent;
use crate::manager::app::{Application, ApplicationState};

macro_rules! safely {
    ($broadcast:expr) => {
        if let Err(e) = $broadcast.await {
            return Arc::new(Err(e));
        }
    };
}

impl Application {
    pub async fn start(&mut self, app_lock: Arc<RwLock<Application>>) -> anyhow::Result<()> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let app_process = self.execute_and_grab_pipes(&app_lock, receiver).await?;
        let app_task = self.spawn_app_task(app_lock, app_process);

        self.state = ApplicationState::Active {
            app_task,
            input_sender: sender,
        };

        Ok(())
    }

    async fn execute_and_grab_pipes(
        &self,
        app_lock: &Arc<RwLock<Application>>,
        receiver: mpsc::Receiver<String>,
    ) -> anyhow::Result<Child> {
        let mut app_process = self.execute().await?;

        let working_directory = self.working_directory().await?;
        let log_file = File::create(working_directory.join("application.log")).await?;
        let shareable_log_file = Arc::new(RwLock::new(log_file));

        self.spawn_stdin_handler(&mut app_process, receiver)?;
        self.spawn_stdout_handler(
            &mut app_process,
            app_lock.clone(),
            shareable_log_file.clone(),
        )?;
        self.spawn_stderr_handler(
            &mut app_process,
            app_lock.clone(),
            shareable_log_file.clone(),
        )?;

        Ok(app_process)
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

    fn spawn_app_task(
        &self,
        app_lock: Arc<RwLock<Application>>,
        mut app_process: Child,
    ) -> JoinHandle<Arc<anyhow::Result<ExitStatus>>> {
        let _enter = self.span.enter();
        let app_span = self.span.clone();

        tokio::spawn(async move {
            let _enter = app_span.enter();

            safely!(send_event(
                &app_lock,
                AppEvent::Start {
                    application: app_lock.clone(),
                }
            ));

            let execution_result = Arc::new(
                app_process
                    .wait()
                    .await
                    .context("error occurred while running application"),
            );

            safely!(send_event(
                &app_lock,
                AppEvent::End {
                    application: app_lock.clone(),
                    result: execution_result.clone(),
                }
            ));

            execution_result
        })
    }

    fn spawn_stdin_handler(
        &self,
        process: &mut Child,
        mut receiver: Receiver<String>,
    ) -> anyhow::Result<()> {
        let child_in = process
            .stdin
            .take()
            .context("new application process lacks stdin")?;

        tokio::spawn(async move {
            let mut stdin_writer = BufWriter::new(child_in);

            loop {
                let receive_result = receiver.recv().await;

                match receive_result {
                    Some(bytes) => match stdin_writer.write_all(bytes.as_bytes()).await {
                        Ok(_) => {}
                        Err(error) => {
                            warn!(?error)
                        }
                    },
                    None => return anyhow::Result::<()>::Ok(()),
                }
            }
        });

        Ok(())
    }
    fn spawn_stdout_handler(
        &self,
        process: &mut Child,
        app_lock: Arc<RwLock<Application>>,
        app_log: Arc<RwLock<File>>,
    ) -> anyhow::Result<()> {
        let child_out = process
            .stdout
            .take()
            .context("new application process lacks stdout")?;

        self.spawn_output_handler(child_out, app_lock, app_log, |application, line| {
            AppEvent::LineOut { application, line }
        })
    }

    fn spawn_stderr_handler(
        &self,
        process: &mut Child,
        app_lock: Arc<RwLock<Application>>,
        app_log: Arc<RwLock<File>>,
    ) -> anyhow::Result<()> {
        let child_out = process
            .stderr
            .take()
            .context("new application process lacks stdout")?;

        self.spawn_output_handler(child_out, app_lock, app_log, |application, line| {
            AppEvent::ErrorLineOut { application, line }
        })
    }

    fn spawn_output_handler<T>(
        &self,
        child_out: T,
        app_lock: Arc<RwLock<Application>>,
        app_log: Arc<RwLock<File>>,
        event_provider: fn(Arc<RwLock<Application>>, String) -> AppEvent,
    ) -> anyhow::Result<()>
    where
        T: AsyncRead + Unpin + Send + 'static,
    {
        tokio::spawn(async move {
            let mut stdout_reader = BufReader::new(child_out);
            let mut line = String::new();

            loop {
                let bytes_read = stdout_reader.read_line(&mut line).await;

                match bytes_read {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            return anyhow::Result::<()>::Ok(());
                        }

                        app_log
                            .write()
                            .await
                            .write_all(line.as_bytes())
                            .await
                            .context("failed to write to log file")?;
                        send_event(&app_lock, event_provider(app_lock.clone(), line.clone()))
                            .await?;
                    }
                    Err(error) => {
                        warn!(?error);
                    }
                }
            }
        });

        Ok(())
    }
}
