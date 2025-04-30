use crate::dal;
use crate::error::AppError;
use crate::init::settings::Settings;
#[double]
use crate::service::hello::Service as HelloService;
use axum::Router;
use mockall_double::double;

/// building blocks that make up the (micro-) service
pub struct BuildingBlocks {
    pub app: Router,
}

impl BuildingBlocks {
    /// initialize and wire up all the dependencies
    pub async fn wire(settings: &Settings) -> Result<BuildingBlocks, AppError> {
        let repo = dal::init(settings.database_url.as_deref()).await?;
        let hello_service = HelloService::new(repo);

        let app = super::super::handler::init(hello_service);

        Ok(BuildingBlocks { app })
    }
}
