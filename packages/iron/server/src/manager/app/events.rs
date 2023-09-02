use std::fmt::Debug;
use std::process::ExitStatus;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::manager::app::Application;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Start {
        application: Arc<RwLock<Application>>,
    },
    End {
        application: Arc<RwLock<Application>>,
        result: Arc<anyhow::Result<ExitStatus>>,
    },
}

#[async_trait]
pub trait AppEventDispatcher: Send + Sync + Debug {
    async fn dispatch(&self, event: AppEvent) -> anyhow::Result<()>;
}
