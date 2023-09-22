use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::manager::app::events::{AppEvent, AppEventHandler};
use crate::manager::app::Application;

#[derive(Debug)]
pub struct LogHandler {
    log_file: RwLock<File>,
}

impl LogHandler {
    pub async fn new(app: &Application) -> anyhow::Result<Self> {
        let working_directory = app.working_directory().await?;
        let log_file = File::create(working_directory.join("application.log")).await?;

        Ok(LogHandler {
            log_file: RwLock::new(log_file),
        })
    }
}

#[async_trait]
impl AppEventHandler for LogHandler {
    async fn handle(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
        match &*event {
            AppEvent::Start { .. } => {
                info!("app started")
            }
            AppEvent::End {
                application: _,
                result,
            } => match &**result {
                Ok(status) => {
                    info!(?status)
                }
                Err(error) => {
                    info!(?error)
                }
            },
            AppEvent::LineOut {
                application: _application,
                line,
            } => {
                info!(line);
                self.log_file
                    .write()
                    .await
                    .write_all(line.as_bytes())
                    .await
                    .context("failed to write to log file")?;
            }
            AppEvent::ErrorLineOut {
                application: _application,
                line,
            } => {
                warn!(line);
                self.log_file
                    .write()
                    .await
                    .write_all(line.as_bytes())
                    .await
                    .context("failed to write to log file")?;
            }
        }

        Ok(())
    }
}
