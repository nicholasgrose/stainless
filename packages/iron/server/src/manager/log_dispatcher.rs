use async_trait::async_trait;
use tracing::{info, span, Level, Span};
use uuid::Uuid;

use crate::manager::app::events::{AppEvent, AppEventDispatcher};

#[derive(Debug)]
pub struct LogDispatcher {
    span: Span,
}

impl LogDispatcher {
    pub fn new(id: &Uuid) -> Self {
        let id = id.to_string();

        LogDispatcher {
            span: span!(Level::INFO, "app task", "uuid" = id),
        }
    }
}

#[async_trait]
impl AppEventDispatcher for LogDispatcher {
    async fn dispatch(&self, event: AppEvent) -> anyhow::Result<()> {
        let _enter = self.span.enter();

        match event {
            AppEvent::Start { .. } => {
                info!("application started")
            }
            AppEvent::End {
                application: _,
                result,
            } => match &*result {
                Ok(status) => {
                    info!("task exited with {}", status)
                }
                Err(error) => {
                    info!("task failed with {}", error)
                }
            },
        }

        Ok(())
    }
}
