use chrono::Local;
use tracing_subscriber::fmt::{time::FormatTime, format::Writer};

pub struct LocalTimer;

impl FormatTime for LocalTimer {
     fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        //write!(w, "{}", Local::now().timestamp_millis())
        write!(w, "{}", Local::now())
    }
}
 