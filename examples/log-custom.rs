use dlog::dlog::{error,warn,info,debug,trace,DLog};

fn main() {
    DLog::new()
        .with_color() // Enable colors in console output (default disabled)
        .widh_timestamp_format("%Y-%m-%d %H:%M:%S") // Change default timestamp
        .widh_custom_separator(" | ") // Change default separator pattern for items
        .with_file("log-custom.log").unwrap() // Enable logging on file (default disable)
        .init_logger().ok();

    error!("Error message");
    warn!("Warning message");
    info!("Info message");
    debug!("Debug message");
    trace!("Trace message");
}