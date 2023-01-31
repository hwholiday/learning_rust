use std::io;

use chrono::prelude::*;
use tracing::info;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

fn main() {
    println!("gateway");
    tracing_subscriber::fmt()
        .json()
        .with_timer(LocalTimer)
        .with_max_level(tracing::Level::TRACE)
        .with_current_span(false)
        .with_writer(io::stdout) // 写入标准输出
        .init();

    let port = 111;
    info!(port, "22"=11,"33"=22,"connected to {:?}", "222");
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        //write!(w, "{}", Local::now().timestamp_millis())
        write!(w, "{}", Local::now())
    }
}
