use dlog::dlog::DLog;

fn main() {
    let dlog=DLog::new();

    println!("{}", dlog.get_status()); // This prints all current crate settings (in this case are defaults)

    dlog.e("Error message");
    dlog.w("Warning message");
    dlog.i("Info message");
    dlog.d("Debug message");
    dlog.t("Trace message");
}