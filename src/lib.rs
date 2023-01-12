#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
//! # DLog
//! 
//! This crate adds logging to your projects or library.
//! 
//! After trying a lot of crates to make logging, I only found crates rich of futures but very complicate to use,
//! or crates that are easy to use but log only on file or only on console.
//! 
//! I decided to write my own lib that aims to be **easy to use with some usefull futures**:
//! * Can log only on stdout, only on file or both.
//! * Very easy to start: install and use immediately.
//! * Short API names.
//! * It implements [log](https://crates.io/crates/log) crate so you can use Rust library logging macros.
//! 
//! ## Add to project:
//! In file cargo.toml add:
//! ```toml
//! [dependencies]
//! dlog = "0.1.0"
//! ```
//! ## Getting Start
//! There are 2 way for use this crate:
//! * Directly create object: ```let dlog=DLog::new();``` and use like ```dlog.d("Log message");```
//! * Initialize logger: ```DLog::new().init_logger().ok();``` and use with [log](https://crates.io/crates/log) macro like ```debug!("Log message");```
//! 
//! Output (on console and/or file) for default settings is like:
//! ```toml
//! 2023/01/02 18.01.27 : DEBUG  : Debug message
//! ```
//! First tag is datetime stamp, second tag is level name followed by log message tag
//! ## Examples:
//! ### Directly usage with default settings:
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
//! This will log on stdout without colors.
//! 
//! ### Directly usage with custom settings:
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
//! This will log on stdout with colors, different formatting for timestamp and different tags separator and in file dlog-custom.log are added same log lines as in console.
//! 
//! ### Macros usage with default settings:
//! ```rust
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
//! ```
//! This will log on stdout without colors.
//! 
//! ### Macros usage with custom settings:
//! ```rust
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
//! This will log on stdout with colors, different formatting for timestamp and different tags separator and in file log-custom.log are added same log lines as in console.
//! 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// the only module in this crate.
pub mod dlog;
