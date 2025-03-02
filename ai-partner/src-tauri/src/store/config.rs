use std::collections::BTreeMap;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::time::Duration;
use std::io::Write;

use config::ConfigError;
use config::{Config, File};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize,Serialize};
use serde_yaml;

const DEFAULT_CONFIG: &str = "setting"; // "Setting.yaml";

#[derive(Deserialize,Serialize,Debug)]
pub struct Configuration {
    pub api: API,
}
impl Configuration {
    pub fn new() -> Result<Configuration,ConfigError> {
        let c = load();
        let configuration = c.try_deserialize()?;
        Ok(configuration)
    }

    pub fn update_api(&mut self, api: API) {
        self.api = api;
        let mut fp = std::fs::File::open(format!("{}.{}",DEFAULT_CONFIG,"yaml")).unwrap();
        let mut data = BTreeMap::new(); 
        data.insert("api.base_url".to_string(), serde_yaml::Value::String(self.api.url.clone()));
        data.insert("api.key".to_string(), serde_yaml::Value::String(self.api.key.clone()));
        data.insert("api.model".to_string(), serde_yaml::Value::String(self.api.model.clone()));
        let yaml_string = serde_yaml::to_string(&data).unwrap();

        fp.write_all(yaml_string.as_bytes()).unwrap();
    }
}
#[derive(Deserialize,Serialize,Debug)]
pub struct  API {
    pub url: String,
    pub key: String,
    pub  model :String
}

fn settings() -> &'static RwLock<Config> {
    static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let settings = load();
        RwLock::new(settings)
    })
}

fn refresh() {
    *settings().write().unwrap() = load();
}

fn load() -> Config {
    Config::builder()
        .add_source(File::with_name("setting.yaml"))
        .build()
        .unwrap()
}

fn show() {
    println!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        settings()
            .read()
            .unwrap()
            .clone()
            .try_deserialize::<Configuration>()
            .unwrap()
    );
}

fn watch() -> ! {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            Path::new("setting.yaml"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(Ok(Event {
                kind: notify::event::EventKind::Modify(_),
                ..
            })) => {
                println!(" * Settings.toml written; refreshing configuration ...");
                refresh();
                show();
            }

            Err(e) => println!("watch error: {e:?}"),

            _ => {
                // Ignore event
            }
        }
    }
}


pub fn update_config<F>(updater: F)
    where F: FnOnce(&mut Config) {
    let mut config = settings().write().unwrap().clone();
    updater(&mut config);
}

pub fn init(){
    show();
}