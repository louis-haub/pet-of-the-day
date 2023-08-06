use std::env;
use env_config::EnvConfig;

#[derive(EnvConfig)]
pub struct Config {
    #[env()]
    pub public_key: String,
}

impl Config {
    pub fn create_and_validate() -> Config {
        let cfg = match Config::from_env(env::vars()) {
            Ok(res) => { res }
            Err(error) => {
                panic!("The following error occurred while initializing Environment: {:#?}", error)
            }
        };
        assert!(cfg.public_key.chars().count() > 0, "PUBLIC_KEY environment variable is missing. Please set it to the public key of your discord app!");

        cfg
    }
}