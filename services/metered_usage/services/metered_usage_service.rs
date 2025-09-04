use std::io::Error;
use tracing::Level;
use tracing::info;
use tracing::instrument;

use crate::infrastructure::clickhouse_client::ClientWrapper;
use crate::{
    repository::metered_usage_repository::MeteredUsageRepository,
    services::entities::MeteredUsageEvent,
};

#[derive(Debug)]
pub struct MeteredUsageService {
    pub repository: MeteredUsageRepository,
    pub db_client: ClientWrapper,
}

impl MeteredUsageService {
    #[instrument(skip_all)]
    pub fn new(db_client: ClientWrapper) -> MeteredUsageService {
        return MeteredUsageService {
            repository: MeteredUsageRepository::new(),
            db_client: db_client,
        };
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub async fn insert_metered_event(
        &self,
        event: MeteredUsageEvent,
        db_client: &ClientWrapper,
    ) -> Result<(), Error> {
        info!("Before inserting");
        self.repository.insert(event.into(), &db_client).await?;
        info!("After inserting");

        return Ok(());
    }
}
