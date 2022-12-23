use dlog::dlog::{error,warn,info,debug,trace,DLog};

fn main() {
    DLog::new().with_color().with_file("./target/debug/examples/test.log").unwrap().init_macro().ok();

    error!("Error message");
    warn!("Warning message");
    debug!("Debug message");
    info!("Info message");
    trace!("Trace message");
}