use dlog::dlog::DLog;

fn main() {
    let dlog=DLog::new()
        .with_color() // Enable colors in console output (default disabled)
        .widh_timestamp_format("%Y-%m-%d %H:%M:%S") // Change default timestamp
        .widh_custom_separator(" | ") // Change default separator pattern for items
        .with_file("dlog-custom.log").unwrap(); // Enable logging on file (default disable)

    println!("{}", dlog.get_status()); // This prints all current crate settings (in this case there are custom)

    dlog.e("Error message");
    dlog.w("Warning message");
    dlog.i("Info message");
    dlog.d("Debug message");
    dlog.t("Trace message");
}