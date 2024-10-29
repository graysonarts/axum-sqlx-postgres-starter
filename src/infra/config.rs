use envconfig::Envconfig;

#[derive(Debug, Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT", default = "3000")]
    pub port: u16,
    #[envconfig(from = "DB_POOL_SIZE", default = "10")]
    pub db_pool_size: u32,
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "FALLBACK_DIR", default = "./public")]
    pub fallback_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self, envconfig::Error> {
        Self::init_from_env()
    }
}
