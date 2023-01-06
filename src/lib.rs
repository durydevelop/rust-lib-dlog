#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
//! # DLog
//! 
//! After try to use a lot of crates to make logging, my conclusion is that there 2 types of logger:
//! 1. Rich of futures but too much complcate to use.
//! 2. Easy to use but single use (e.g. log only on file or only on console).
//! So, I decided to write my own lib that aims to be easy to use but with with some usefull futures:
//! * Log on stdout.
//! * Log on file.
//! * Very easy to start (default standard settings in new() method).
//! * Easy to call.
//! * Implements [log](https://crates.io/crates/log) crate to use Rust library logging macros.
//! 
//! ```toml
//! [dependencies]
//! tui = "0.19"
//! crossterm = "0.25"
//! ```
//! 
//! ## Examples:
//! 
//! ### Direct usage:
//! 
//! Default settings usage:
//! 
//! ```rust
//! use dlog::dlog::DLog;
//! 
//! fn main() {
//!     let dlog=DLog::new();
//! 
//!     println!("{}", dlog.get_status()); // This prints all current crate settings (in this case are defaults)
//! 
//!     dlog.e("Error message");
//!     dlog.w("Warning message");
//!     dlog.i("Info message");
//!     dlog.d("Debug message");
//!     dlog.t("Trace message");
//! }
//! ```
//! This will outputs:
//! 
//!
//! ----------- dlog current settings -----------
//! 
//! Show Colors       =  false
//! 
//! Show Level        =  true
//! 
//! Show Timestamp    =  true
//! 
//! Timestamp Format  =  %Y/%m/%d %H.%M.%S
//! 
//! Tags separator    =  ' : '
//! 
//! Level             =  TRACE
//! 
//! Log on stdout     =  true
//! 
//! Log on file       =  false
//! 
//! Max file size     =  no limit
//! 
//! Max files count   =  no limit
//! 
//! ---------------------------------------------
//! 
//! 2023/01/01 12.43.14 : ERROR  : Error message
//! 
//! 2023/01/01 12.43.14 : WARN   : Warning message
//! 
//! 2023/01/01 12.43.14 : INFO   : Info message
//! 
//! 2023/01/01 12.43.14 : DEBUG  : Debug message
//! 
//! 2023/01/01 12.43.14 : TRACE  : Trace message
//! 
//! 
//! Custom settimgs usage:
//! 
//! ```rust
//! use dlog::dlog::DLog;
//! 
//! fn main() {
//!     let dlog=DLog::new()
//!     .with_color() // Enable colors in console output (default disabled)
//!     .widh_timestamp_format("%Y-%m-%d %H:%M:%S") // Change default timestamp
//!     .widh_custom_separator(" | ") // Change default separator pattern for items
//!     .with_file("dlog-custom.log").unwrap(); // Enable logging on file (default disable)
//! 
//! println!("{}", dlog.get_status()); // This prints all current crate settings (in this case there are custom)
//! 
//!     dlog.e("Error message");
//!     dlog.w("Warning message");
//!     dlog.i("Info message");
//!     dlog.d("Debug message");
//!     dlog.t("Trace message");
//! }
//! ```
//! 
//! This will outputs:
//! 
//! ----------- dlog current settings -----------
//! 
//! Show Colors       =  true
//! 
//! Show Level        =  true
//! 
//! Show Timestamp    =  true
//! 
//! Timestamp Format  =  %Y-%m-%d %H:%M:%S
//! 
//! Tags separator    =  ' | '
//! 
//! Level             =  TRACE
//! 
//! Log on stdout     =  true
//! 
//! Log on file       =  true
//! 
//! Current filename  =  \\?\E:\Dev\rust\lib\dlog\dlog-custom.log
//! 
//! Max file size     =  no limit
//! 
//! Max files count   =  no limit
//! 
//! ---------------------------------------------
//! 
//! 2023-01-02 18:00:02 | ERROR  | Error message
//! 
//! 2023-01-02 18:00:02 | WARN   | Warning message
//! 
//! 2023-01-02 18:00:02 | INFO   | Info message
//! 
//! 2023-01-02 18:00:02 | DEBUG  | Debug message
//! 
//! 2023-01-02 18:00:02 | TRACE  | Trace message
//! 
//! 
//! In file dlog-custom.log are added same log lines as in console.
//! 
//! ### Macros usage:
//! 
//! Default settings version:
//! 
//! ```
//! use dlog::dlog::{error,warn,info,debug,trace,DLog};
//! 
//! fn main() {
//!     DLog::new().init_logger().ok();
//! 
//!     error!("Error message");
//!     warn!("Warning message");
//!     info!("Info message");
//!     debug!("Debug message");
//!     trace!("Trace message");
//! }
//! 
//! This will outputs:
//! 
//! 2023/01/02 18.01.27 : ERROR  : Error message
//! 2023/01/02 18.01.27 : WARN   : Warning message
//! 2023/01/02 18.01.27 : INFO   : Info message
//! 2023/01/02 18.01.27 : DEBUG  : Debug message
//! 2023/01/02 18.01.27 : TRACE  : Trace message
//! 
//! 
//! Custom settings version:
//! ```
//! use dlog::dlog::{error,warn,info,debug,trace,DLog};
//! 
//! fn main() {
//!     DLog::new()
//!         .with_color() // Enable colors in console output (default disabled)
//!         .widh_timestamp_format("%Y-%m-%d %H:%M:%S") // Change default timestamp
//!         .widh_custom_separator(" | ") // Change default separator pattern for items
//!         .with_file("log-custom.log").unwrap() // Enable logging on file (default disable)
//!         .init_logger().ok();
//! 
//!     error!("Error message");
//!     warn!("Warning message");
//!     info!("Info message");
//!     debug!("Debug message");
//!     trace!("Trace message");
//! }
//! ```
//! 
//! This will outputs:
//! 
//! 2023-01-02 18:02:02 | ERROR  | Error message
//! 2023-01-02 18:02:02 | WARN   | Warning message
//! 2023-01-02 18:02:02 | INFO   | Info message
//! 2023-01-02 18:02:02 | DEBUG  | Debug message
//! 2023-01-02 18:02:02 | TRACE  | Trace message
//! 
//! In file log-custom.log are added same log lines as in console.
//! 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod dlog;
