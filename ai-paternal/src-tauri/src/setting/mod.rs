use std::io::Write;

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

const DEFAULT_CONFIG: &str = "config";

#[derive(Debug, Deserialize,Default)]
#[allow(unused)]
pub struct API {
    pub base_url: String,
    pub key:String,
    pub model:String,
}

#[derive(Debug, Deserialize,Default)]
#[allow(unused)]
pub struct Settings {
    pub api: API
}


fn load() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(File::with_name(DEFAULT_CONFIG))
        .build()
}
use std::collections::BTreeMap;
use serde_yaml;
impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let s = load()?;
        // You can deserialize (and thus freeze) the entire configuration as
        let mut ss:Self = s.try_deserialize()?;
        ss.handle_base_url();
        Ok(ss)
    }

    fn handle_base_url(&mut self) {
        self.api.base_url = self.api.base_url.replace("/v1", "/v1/chat/completions");
    }
    pub fn update_api(&mut self,api :API){
        self.api = api;
        let mut fp = std::fs::File::open(format!("{}.{}",DEFAULT_CONFIG,"yaml")).unwrap();
        let mut data = BTreeMap::new(); 
        data.insert("api.base_url".to_string(), serde_yaml::Value::String(self.api.base_url.clone()));
        data.insert("api.key".to_string(), serde_yaml::Value::String(self.api.key.clone()));
        data.insert("api.model".to_string(), serde_yaml::Value::String(self.api.model.clone()));
        let yaml_string = serde_yaml::to_string(&data).unwrap();
        fp.write_all(yaml_string.as_bytes()).unwrap();
    }
}