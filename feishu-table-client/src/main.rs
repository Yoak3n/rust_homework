use tokio;
use feishu_table_client::{
    process_data::{
        request_data,
        parese_data
    },
    args_parse::*,
    fetch_data
};

#[tokio::main]
async fn main() {
    let args = parse_config().unwrap();
    let device = fetch_data::new();
    let data = parese_data(&device).unwrap();
    // let parsed: Value = serde_json::from_str(&args.data).unwrap();
    let res = request_data(args.server,data,args.token).await.unwrap();
    println!("{}",res);
}




