use std::{io::{self, Result}, fs::OpenOptions};

use dlog::dlog::{debug,DLog};


fn main() {
    DLog::new().with_file("test.log").unwrap().init_macro().ok();
    debug!("Secondo");
}