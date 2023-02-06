
use crate::store;
use crate::configuration::AppConfig;

pub struct App<DB: store::Store> {
    pub store: DB,
    pub config: AppConfig,
}

impl<DB: store::Store + Clone> App<DB> {
    fn new(config: AppConfig, store: DB) -> App<DB> {
        return App {
            config,
            store
        }
    }
}

impl<DB: store::Store + Clone> Clone for App<DB> {
    fn clone(&self) -> Self {
        ***self
    }
}