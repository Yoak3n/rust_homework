
use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize,Default)]
#[allow(unused)]
pub struct API {
    pub base_url: String,
    pub key:String
}

#[derive(Debug, Deserialize,Default)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    pub api: API
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config").required(true))
            .build()?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("api.base_url"));
        // You can deserialize (and thus freeze) the entire configuration as
        let mut ss:Self = s.try_deserialize()?;
        ss.handle_base_url();
        Ok(ss)
    }

    pub fn handle_base_url(&mut self) {
        self.api.base_url = self.api.base_url.replace("/v1", "/v1/chat/completions");
    }
}