use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::manager::app::events::{AppEvent, AppEventHandler, AppEventType, LineType};
use crate::manager::app::Application;

#[derive(Debug)]
pub struct LogHandler {
    log_file: RwLock<File>,
}

impl LogHandler {
    pub async fn new(app: &Application) -> anyhow::Result<Self> {
        Ok(LogHandler {
            log_file: RwLock::new(create_log_file(app).await?),
        })
    }
}

async fn create_log_file(app: &Application) -> anyhow::Result<File> {
    let app_directory = &app.config.directory;
    let log_file_path = app_directory.join("application.log");
    tokio::fs::create_dir_all(app_directory).await?;
    let log_file = File::create(log_file_path).await?;

    Ok(log_file)
}

#[async_trait]
impl AppEventHandler for LogHandler {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &event.event_type {
            AppEventType::Start { .. } => {
                info!("app started")
            }
            AppEventType::End { result } => match &**result {
                Ok(status) => {
                    info!(?status)
                }
                Err(error) => {
                    info!(?error)
                }
            },
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

                self.log_file
                    .write()
                    .await
                    .write_all(text.as_bytes())
                    .await
                    .context("failed to write to log file")?;
            }
        }

        Ok(())
    }
}
