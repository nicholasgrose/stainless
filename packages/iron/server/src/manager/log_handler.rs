use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::manager::app::events::{AppEvent, AppEventHandler, AppEventType, LineType};
use crate::manager::app::Application;

#[derive(Debug)]
pub struct LogHandler {
    log_file_path: PathBuf,
    state: RwLock<LogHandlerState>,
}

#[derive(Debug)]
struct LogHandlerState {
    file_writer: Option<BufWriter<File>>,
}

impl LogHandler {
    pub async fn new(app: &Application) -> anyhow::Result<Self> {
        let app_directory = &app.config.directory;
        let log_file_path = app_directory.join("application.log");

        Ok(LogHandler {
            log_file_path,
            state: RwLock::new(LogHandlerState { file_writer: None }),
        })
    }

    async fn open_writer(&self) -> anyhow::Result<()> {
        let mut state = self.state.write().await;

        match &state.file_writer {
            None => {
                state.open_writer(&self.log_file_path).await?;

                Ok(())
            }
            Some(_) => Ok(()),
        }
    }
}

impl LogHandlerState {
    async fn open_writer(&mut self, path: &PathBuf) -> anyhow::Result<()> {
        let log_file = self.create_log_file(path).await?;
        let log_file_writer = BufWriter::new(log_file);

        self.file_writer = Some(log_file_writer);

        Ok(())
    }

    async fn create_log_file(&self, path: &PathBuf) -> anyhow::Result<File> {
        tokio::fs::create_dir_all(path).await?;
        let log_file = File::create(path).await?;

        Ok(log_file)
    }

    fn close_writer(&mut self) {
        self.file_writer = None;
    }
}

#[async_trait]
impl AppEventHandler for LogHandler {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &event.event_type {
            AppEventType::Start { .. } => {
                self.open_writer().await?;
                info!("app started")
            }
            AppEventType::End { result } => {
                match &**result {
                    Ok(status) => {
                        info!(?status);
                    }
                    Err(error) => {
                        warn!(?error);
                    }
                }

                self.state.write().await.close_writer();
            }
            AppEventType::Print { line } => {
                let text = match line {
                    LineType::Out(out_line) => {
                        info!(?line);
                        out_line
                    }
                    LineType::Error(err_line) => {
                        warn!(?line);
                        err_line
                    }
                };

                let mut state = self.state.write().await;

                match &mut state.file_writer {
                    None => {
                        state.open_writer(&self.log_file_path).await?;
                    }
                    Some(writer) => {
                        writer
                            .write_all(text.as_bytes())
                            .await
                            .context("failed to write to log file")?;
                    }
                };
            }
        }

        Ok(())
    }
}
