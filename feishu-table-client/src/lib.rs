pub mod fetch_data;
pub mod args_parse;

pub mod process_data{
    use serde_json::{from_str,Value};
    use crate::fetch_data::DeviceInfo;
    use std::error::Error;

    pub fn parese_data(data: &DeviceInfo) -> Result<Value,Box<dyn Error>> {
        
        let f = format!(r#"{{
            "家庭设备":"{}","内存占用":{},"磁盘占用":{},"设备类型":"{}","CPU占用":{}
        }}"#,data.name,data.memory_usage,data.disk_usage,data.device_type,data.cpu_usage);
        let record: Value = from_str(&f)?;
        Ok(record)
    }

    pub async fn request_data(uri:String,data: Value,token :String,) ->Result<String,Box<dyn Error>>{
        let client = reqwest::Client::new();
        let data = data.as_object().unwrap();
        let res = client
        .post(uri)
        .json(&data)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .text()
        .await?;
    
        Ok(res)
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test_get_sysinfo(){
        let d = fetch_data::new();
        println!("{}",d.name);
        assert_eq!(d.name,"Yoake3");
    }
    
}
