use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Deserialize)]
pub struct ProxyRequest {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

#[derive(Serialize)]
pub struct ProxyResponse {
    status: u16,
    body: String,
}

// 这个命令在 Rust 这一侧发起 HTTP 请求，不受浏览器同源策略/CORS 限制，
// 专门用来绕开 OpenAI 等服务商"不允许浏览器直接调用"的限制。
// 前端调用方式：invoke('http_proxy', { req: { url, method, headers, body } })
#[tauri::command]
async fn http_proxy(req: ProxyRequest) -> Result<ProxyResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(180))
        .build()
        .map_err(|e| e.to_string())?;

    let method = req.method.to_uppercase();
    let mut builder = match method.as_str() {
        "GET" => client.get(&req.url),
        "POST" => client.post(&req.url),
        "PUT" => client.put(&req.url),
        "DELETE" => client.delete(&req.url),
        other => return Err(format!("不支持的请求方法：{}", other)),
    };

    for (key, value) in req.headers.iter() {
        builder = builder.header(key, value);
    }

    if let Some(body) = req.body {
        builder = builder.body(body);
    }

    let response = builder
        .send()
        .await
        .map_err(|e| format!("请求发送失败：{}", e))?;

    let status = response.status().as_u16();
    let body = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败：{}", e))?;

    Ok(ProxyResponse { status, body })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![http_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
