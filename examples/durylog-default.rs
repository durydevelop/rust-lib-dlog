use durylog::DLog;

fn main() {
    let durylog=DLog::new();

    println!("{}", durylog.get_status()); // This prints all current crate settings (in this case are defaults)

    durylog.e("Error message");
    durylog.w("Warning message");
    durylog.i("Info message");
    durylog.d("Debug message");
    durylog.t("Trace message");
}