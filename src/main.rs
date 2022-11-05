mod config;

use anyhow::Result;

use actix_web::{get, web, App, HttpServer, Responder};
use tracing::info;

use crate::config::Conf;

#[get("/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("日志模块初始化成功");
    Conf::init_conf();
    info!("全局配置导入成功");
    let server = HttpServer::new(|| App::new().service(greet))
        .bind(format!("{}:{}", Conf::global().ip, Conf::global().port))?
        // disable default signal handling
        .disable_signals()
        .run();
    info!("网络模块初始化成功");
    info!("绑定地址: {}:{}", Conf::global().ip, Conf::global().port);

    let server_handle = server.handle();

    let server_task = tokio::spawn(server);

    let shutdown = tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("接收到 Ctrl-C 开始终止服务");

        let server_stop = server_handle.stop(true);
        server_stop.await;
        info!("服务成功终止");
    });

    let _ = tokio::try_join!(server_task, shutdown).expect("添加任务失败");

    Ok(())
}
