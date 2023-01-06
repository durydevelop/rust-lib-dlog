use dlog::dlog::{error,warn,info,debug,trace,DLog};

fn main() {
    DLog::new().init_logger().ok();

    error!("Error message");
    warn!("Warning message");
    info!("Info message");
    debug!("Debug message");
    trace!("Trace message");
}