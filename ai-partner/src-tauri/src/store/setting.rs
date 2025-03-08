use serde::{Deserialize,Serialize};
use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::RwLock;
use std::fs::write;
const DEFAULT_CONFIG: &str = "setting"; // "Setting.yaml";

static CONFIG: Lazy<RwLock<Configuration>> = Lazy::new(|| {
    let default_config: Configuration = Configuration {
        api: API {
            url: "".into(),
            key: "".into(),
            model: "".into(),
        },
        smooth: false
    };
    RwLock::new(default_config)
});

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct  API {
    pub url: String,
    pub key: String,
    pub model :String
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Configuration {
    pub api: API,
    pub smooth: bool,
}
impl Configuration {
    fn default() -> Self {
        Configuration {
            api: API {
                url: "https://api.example.com".into(),
                key: "your-api-key".into(),
                model: "default-model".into(),
            },
            smooth: false
        }
    }
    // 初始化配置（从文件加载）
    pub fn init_config() -> Result<Configuration, Box<dyn std::error::Error>> {
        let config_path = format!("{}.yaml", DEFAULT_CONFIG);
        
               // 检查配置文件是否存在
        if !Path::new(&config_path).exists() {
            // 创建默认配置
            let default_config = Configuration::default();
            let yaml_content = serde_yaml::to_string(&default_config)?;
            write(&config_path, yaml_content)?;
            println!("Created default config file");
        }

        let config_content = std::fs::read_to_string(&config_path)?;
        let new_config: Configuration = serde_yaml::from_str(&config_content)?;
        println!("Loaded config: {:?}", new_config);
        
        // 获取写锁并更新配置
        let mut config = CONFIG.write().unwrap();
        *config = new_config.clone();
        Ok(new_config)


    }
    pub fn get_config() -> Self {
        CONFIG.read().unwrap().clone()
    }

    pub fn update_config<F>(updater: F) ->Result<(),Box<dyn std::error::Error>>
        where F: FnOnce(&mut Configuration) {

        let old_config = CONFIG.read().unwrap().clone();
        let mut config = CONFIG.write().unwrap();
        updater(&mut config);
        match serde_yaml::to_string(&*config)
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
            .and_then(|data| {
                println!("Saving config: {:?}", data);
                write(format!("{}.yaml", DEFAULT_CONFIG),data)
                .map_err(|e:std::io::Error|e.into())
            }){
                Ok(_) => return Ok(()),
                Err(e) => {
                    *config = old_config;
                    eprintln!("Failed to save config: {}", e);
                    return Err(e);
            }
        }
    }
}

