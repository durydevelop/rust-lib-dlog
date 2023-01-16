use durylog::DLog;

fn main() {
    let durylog=DLog::new()
        .with_color() // Enable colors in console output (default disabled)
        .widh_timestamp_format("%Y-%m-%d %H:%M:%S") // Change default timestamp
        .widh_custom_separator(" | ") // Change default separator pattern for items
        .with_file("durylog-custom.log").unwrap(); // Enable logging on file (default disable)

    println!("{}", durylog.get_status()); // This prints all current crate settings (in this case there are custom)

    durylog.e("Error message");
    durylog.w("Warning message");
    durylog.i("Info message");
    durylog.d("Debug message");
    durylog.t("Trace message");
}