use ratatui::text;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]  // 使用 Clone
struct message_list {
    status: String,
    retcode: i32,
    data: message_data,
    message: String,
    wording: String,
    echo: Option<String>,
}

#[derive(Deserialize, Clone)]
struct message_data {
    messages: Vec<message>
}

#[derive(Deserialize, Clone)]
struct message {
    self_id: u32,
    user_id: u32,
    time: u32,
    message_id: u32,
    message_seq: u32,
    real_id: u32,
    real_seq: String,
    message_type: String,
    sender: sender_info,
    raw_message: String,
    font: u32,
    sub_type: String,
    message: Vec<message_content>,
    message_format: String,
    post_type: String,
    group_id: u64,
}

#[derive(Deserialize, Clone)]
struct sender_info {
    user_id: u32,
    nickname: String,
    card: String,
    role: String,
}

#[derive(Deserialize, Clone)]
struct message_content {
    r#type: String,
    data: text_message,
}

#[derive(Deserialize, Clone)]
struct text_message {
    text: String
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let send_group_msg_json = serde_json::json!({
        "group_id": "1055065019",
        "message": [
            {
                "type": "text",
                "data": {
                    "text": "Hello, this is a test message from Rust!"
                }
            }
        ]
        
    });
    let data = serde_json::json!({
        "group_id": "979885968",
        "message_seq": "7591476223401494106",
        "count": 2,
        "reverseOrder": true
        
    });
    
    // 先获取原始响应文本，用于调试
    let response_text = client
        .post("http://10.29.237.29:3001/get_group_msg_history")
        .json(&data)
        .send()
        .await?
        .text()
        .await?;
    
    println!("原始响应: {}", response_text);

    // 解析消息列表
    print!("解析后的消息列表:\n");
    parse_msg_list(response_text)?;
    
    Ok(())
}

fn parse_msg_list(mgs_list_json: String) -> Result<(), Box<dyn std::error::Error>> {
    let msg_list:message_list = serde_json::from_str(&mgs_list_json)?;
    let user_name = String::from(&msg_list.data.messages[1].sender.nickname);
    let msg_content = String::from(&msg_list.data.messages[1].raw_message);

    println!("用户: {}, 消息内容: {}", user_name, msg_content);
    Ok(())
}