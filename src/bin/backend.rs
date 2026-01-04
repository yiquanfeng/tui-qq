use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let data = serde_json::json!({
        "group_id": "979885968",
        "message_seq": "7591036332879274936",
        "count": 10,
        "reverseOrder": false
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
    
    Ok(())
}