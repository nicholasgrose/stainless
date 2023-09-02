use std::fmt::Debug;

use async_trait::async_trait;

use crate::manager::app::Application;

#[derive(Debug)]
pub enum AppEvent<'a> {
    Start { application: &'a Application },
    End { application: &'a Application },
}

#[async_trait]
pub trait AppEventDispatcher: Send + Sync + Debug {
    async fn dispatch<'a>(&self, event: AppEvent<'a>) -> anyhow::Result<()>;
}
