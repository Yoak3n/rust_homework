use serde::{Deserialize,Serialize};


#[allow(dead_code)]
#[derive(Serialize,Deserialize,Clone)]
pub struct MessageItem{
    pub role: String,
    pub content: String,
    pub reasoning_content: String,
}
impl Default for MessageItem {
    fn default() -> Self {
        Self {
            role: "assistant".to_string(),
            content: "".to_string(),
            reasoning_content: "".to_string(),
        }
    }
}

impl MessageItem {
    // pub fn new(role: String, content: String, reasoning_content: Option<String>) -> Self {
    //     Self {
    //         role,
    //         content,
    //         reasoning_content,
    //     }
    // }
    pub fn append(&mut self, content:&MessageType) {
        match content {
            MessageType::Content(c) => {
                self.content.push_str(c);
            },
            MessageType::ReasoningContent(r)=>{
                self.reasoning_content.push_str(r);
            
        }
    }
    }
}


pub enum MessageType {
    ReasoningContent(String),
    Content(String),
}
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct StreamMessageItem{
    pub role: Option<String>,
    pub content: Option<String>,
    pub reasoning_content: Option<String>,


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
#[derive(Serialize,Clone)]
pub struct StreamEmitter{
    pub message_type : String,
    pub data: String,
    pub index: usize,
    pub id: usize,
}

impl StreamEmitter{
    pub fn new(message_type: MessageType, index: usize, id: usize) -> Self {
        let (message_type,data) = match message_type {
            MessageType::ReasoningContent(content) => ("reasoning_content".to_string(),content),
            MessageType::Content(content) =>  ("content".to_string(),content)
        };
        Self{
            message_type,
            data,
            index,
            id
        }
    }
}