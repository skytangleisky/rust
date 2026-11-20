// 文件路径：src/main.rs
use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 请求参数结构体（反序列化）
#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>, // Option 表示可选参数
}

// 响应结构体（序列化）
#[derive(Serialize)]
struct ApiResponse {
    code: u16,
    message: String,
    data: Option<HashMap<String, String>>, // 可选的数据字段
}

// 处理 GET /hello 请求
// Query(HelloParams) 自动从 URL 查询参数中提取 name
async fn hello_handler(Query(params): Query<HelloParams>) -> Json<ApiResponse> {
    // let name = params.name.unwrap_or_else(|| String::from("RUNOOB"));

    // let mut data = HashMap::new();
    // data.insert("name".to_string(), name.clone());

    // Json(ApiResponse {
    //     code: 200,
    //     message: format!("你好，{}！欢迎访问 Rust HTTP 服务", name),
    //     data: Some(data),
    // })

    let name = params.name.unwrap_or_else(|| "RUNOOB".to_string());
    Json(ApiResponse {
        code: 200,
        message: format!("你好，{}！欢迎访问 Rust HTTP 服务", name),
        data:None,
    })
}

// #[tokio::main] 宏将 async fn main 转换为 Tokio 运行时
#[tokio::main]
async fn main() {
    // 构建路由
    let app = Router::new()
        .route("/hello", get(hello_handler));

    // 绑定地址并启动服务
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000")
        .await
        .unwrap();

    println!("RUNOOB 服务器启动，监听地址: http://localhost:9000");
    println!("访问示例: http://localhost:9000/hello?name=Rust开发者");

    axum::serve(listener, app).await.unwrap();
}