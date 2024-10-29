use sqlx::PgPool;

use super::{db::make_pool, Config};

pub struct AppState {
    pub pool: PgPool,
} // Replace with your appstate

impl AppState {
    pub async fn from_config(config: &Config) -> Result<Self, anyhow::Error> {
        let pool = make_pool(config).await?;
        Ok(Self { pool })
    }
}
