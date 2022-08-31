use std::env;
use std::sync::Arc;

pub type ConfigHandle = Arc<Config>;

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub auth_key: String,
    pub service: bool,
    pub env: Environment,
    _private: (),
}

impl Config {
    pub fn new(
        port: u16,
        database_url: impl Into<String>,
        auth_key: impl Into<String>,
        service: bool,
        env: Environment,
    ) -> ConfigHandle {
        Arc::new(Self {
            port,
            database_url: database_url.into(),
            auth_key: auth_key.into(),
            service,
            env,
            _private: (),
        })
    }

    pub fn from_env() -> ConfigHandle {
        Arc::new(Self {
            port: env::var("PORT")
                .expect("Could not find env PORT")
                .parse()
                .expect("Invalid value for PORT"),
            database_url: env::var("DATABASE_URL").expect("Could not find env DATABASE_URL"),
            auth_key: env::var("AUTH_KEY").expect("Could not find env AUTH_KEY"),
            service: match env::var("SERVICE").ok().as_deref() {
                Some("true") | Some("TRUE") | None => true,
                Some("false") | Some("FALSE") => false,
                Some(env) => panic!("Invalid value for SERVICE: {}", env),
            },
            env: match env::var("ENVIRONMENT")
                .expect("Could not find env ENVIRONMENT")
                .as_str()
            {
                "Production" => Environment::Production,
                "Development" => Environment::Development,
                "Test" => Environment::Test,
                env => panic!("Invalid value for ENVIRONMENT: {}", env),
            },
            _private: (),
        })
    }
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Development,
    Test,
}
