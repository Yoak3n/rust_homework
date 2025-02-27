use serde::{Deserialize,Serialize};
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct StreamMessageItem{
    pub role: Option<String>,
    pub content: String,


}
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Choice{
    pub delta: StreamMessageItem,
    pub index:usize,
    pub logprobs: Option<String>,
    pub finish_reason: Option<String>,
}
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct StreamData{
    pub choices: Vec<Choice>,
    pub object: Option<String>,
    pub usage: Option<String>,
    pub created: Option<usize>,
    pub system_fingerprint: Option<String>,
    pub id: Option<String>,
    pub model: Option<String>,
}
#[allow(dead_code)]
#[derive(Serialize,Deserialize,Clone)]
pub struct MessageItem{
    pub role: String,
    pub content: String,
    pub reasoning_content: Option<String>,
}