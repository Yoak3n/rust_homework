use std::sync::{Mutex,Arc};
use crate::store::setting::Configuration;
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AppState {
    pub config :Arc<Mutex<Configuration>>
}