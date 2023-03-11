use std::io;

pub mod pkg;

pub fn setup(){
    tracing_subscriber::fmt()
        .json()
        .with_timer(pkg::log::LocalTimer)
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .with_target(false)
        .with_writer(io::stdout) // 写入标准输出
        .init();

}