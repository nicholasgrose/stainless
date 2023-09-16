use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, info_span, instrument, Span};
use uuid::Uuid;

use crate::manager::app::events::{AppEvent, AppEventDispatcher};

#[derive(Debug)]
pub struct LogDispatcher {
    span: Span,
}

impl LogDispatcher {
    pub fn new(uuid: &Uuid) -> Self {
        LogDispatcher {
            span: info_span!("app task", ?uuid),
        }
    }
}

#[async_trait]
impl AppEventDispatcher for LogDispatcher {
    #[instrument(parent = &self.span)]
    async fn dispatch(&self, event: Arc<AppEvent>) -> anyhow::Result<()> {
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
        }

        Ok(())
    }
}
