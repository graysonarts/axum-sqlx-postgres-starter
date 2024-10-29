use sqlx::{postgres::PgPoolOptions, PgPool};

use super::Config;

pub async fn make_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    let options = PgPoolOptions::new().max_connections(config.db_pool_size);
    options.connect(&config.database_url).await
}
