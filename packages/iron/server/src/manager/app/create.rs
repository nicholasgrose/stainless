use tokio::sync::{broadcast, RwLock};
use tracing::info_span;

use crate::manager::app::{
    AppConfig, AppEventHandlers, AppEvents, AppProperties, AppRunState, AppState, Application,
};

#[derive(Debug)]
pub struct AppCreationSettings {
    pub properties: AppProperties,
    pub handlers: AppEventHandlers,
}

const EVENT_CHANNEL_SIZE: usize = 16;

impl From<AppCreationSettings> for Application {
    fn from(settings: AppCreationSettings) -> Self {
        let (sender, _receiver) = broadcast::channel(EVENT_CHANNEL_SIZE);

        Application {
            config: AppConfig {
                span: info_span!(parent: None, "app", ?settings.properties).into(),
                directory: format!("{}_{}", settings.properties.name, settings.properties.id)
                    .into(),
                properties: settings.properties,
            },
            events: AppEvents {
                async_channel: sender,
                handlers: settings.handlers.into(),
            },
            state: RwLock::new(AppState {
                run_state: AppRunState::NotStarted,
            }),
        }
    }
}
