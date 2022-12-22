//! # Usage
//! ```rust
//! use logs::{debug, error, info, trace, warn, Logs};
//!
//! Logs::new().init();
//! trace!("This is a trace log");
//! debug!("This is a debug log");
//! info!("This is a info log");
//! warn!("This is a warn log");
//! error!("This is a error log");
//! ```
//! Output:
//! ```ignore
//! 2022-09-06T08:38:23.490 [TRACE] This is a trace log
//! 2022-09-06T08:38:23.490 [DEBUG] This is a debug log
//! 2022-09-06T08:38:23.490 [INFO ] This is a info log
//! 2022-09-06T08:38:23.490 [WARN ] This is a warn log
//! 2022-09-06T08:38:23.490 [ERROR] This is a error log
//! ```
//! ## Options
//! ```ignore
//! use logs::{Logs, debug, error, LevelFilter};
//! Logs::new()
//!     TODO
//!     // Apply
//!     .init();
//! ```
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
