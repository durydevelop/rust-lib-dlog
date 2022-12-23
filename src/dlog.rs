pub use log::{debug, error, info, trace, warn, LevelFilter};
use log::{Level, Log, Metadata, ParseLevelError, Record, SetLoggerError};
use std::{
    env::{self, VarError},
    str::FromStr, fs::{OpenOptions, File, self}, io::{self, Write, stdout}, path::{Path, PathBuf},
};
use time::{format_description::FormatItem, OffsetDateTime};

/// Levels constants
const ERROR_COLOR: &str = "\x1B[31mERROR \x1B[0m";
const WARN_COLOR: &str = "\x1B[33mWARN  \x1B[0m";
const INFO_COLOR: &str = "\x1B[32mINFO  \x1B[0m";
const DEBUG_COLOR: &str = "\x1B[3;34mDEBUG \x1B[0m";
const TRACE_COLOR: &str = "\x1B[2;3mTRACE \x1B[0m";
const ERROR: &str = "ERROR ";
const WARN: &str =  "WARN  ";
const INFO: &str =  "INFO  ";
const DEBUG: &str = "DEBUG ";
const TRACE: &str = "TRACE ";

/// Timestamp format definition
const TIMESTAMP_FORMAT: &[FormatItem] = time::macros::format_description!(
    "[year]/[month]/[day]T[hour].[minute].[second].[subsecond digits:3]"
);

/// Info separator
const SEP: &str = " : ";


//const MB: u64 = 1024 * 1024;

#[derive(Debug)]
pub enum DLogError {
    Level(ParseLevelError),
    Env(VarError),
    Err(io::Error),
    None,
}
/*
enum DStorageMode {
    BySize,
    ByTime,
}
*/
#[derive(Debug)]
pub struct DLog {
    level: LevelFilter,
    target: Option<String>,

    color_enabled: bool,
    log_on_stdout: bool,

    // File params
    log_on_file: bool,
    filename: PathBuf,
    file: Option<File>,
    max_file_size: u64,
    max_files_count: u64,

    // Formatting message flags
    timestamp_enabled: bool,
    level_enabled: bool,
}

impl Default for DLog {
    fn default() -> Self {
        Self::new()
    }
}

impl DLog {
    
    pub fn new() -> Self {
        Self {
            level: LevelFilter::Trace,
            target: None,

            color_enabled: false,
            log_on_stdout: true,

            log_on_file:false,
            filename: PathBuf::new(),
            file: None,
            max_file_size: 0, // no limits
            max_files_count: 0, // no limits

            timestamp_enabled: true,
            level_enabled: true,
        }
    }
    
// ********** new() default modification functions **********
    /// Enable logging on file and open it
    pub fn with_file(mut self, filename: &str) -> Result<Self, DLogError> {
        match self.open_file(filename) {
            Ok(file) => {
                self.file=Some(file);
                self.log_on_file=true;
                self.filename=PathBuf::from(filename);
                Ok(self)
                
            },
            Err(err) => {
                self.file=None;
                self.log_on_file=false;
                self.filename.clear();
                Err(DLogError::Err(err))
            }
        }
    }

    /// Convenient function to enable color in construction.
    pub fn with_color(mut self) -> Self {
        self.enabled_colors(true);
        self
    }

    /// Filter log target
    pub fn target_filter<S: AsRef<str>>(mut self, target: S) -> Self {
        let target = target.as_ref().replace('-', "_");
        self.target = Some(target);
        self
    }


    /// Filter log level
    pub fn level(&mut self, level: LevelFilter) -> &mut Self {
        self.level = level;
        self
    }

    /// Filter log level from ['name'] environment variable
    pub fn level_from_env<S: AsRef<str>>(self, name: S) -> Result<Self, DLogError> {
        match env::var(name.as_ref()) {
            Ok(s) => self.level_from_str(&s),
            Err(err) => Err(DLogError::Env(err)),
        }
    }

    /// Filter log level from `str`
    pub fn level_from_str<S: AsRef<str>>(mut self, s: S) -> Result<Self, DLogError> {
        match LevelFilter::from_str(s.as_ref()) {
            Ok(level) => {
                self.level = level;
                Ok(self)
            }
            Err(err) => Err(DLogError::Level(err)),
        }
    }

    /// Complete initialization.
    /// Must call before use std::log macro.
    pub fn init_macro(self) -> Result<(),SetLoggerError> {
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(LevelFilter::Trace))
    }

// **********************************************************

// ******************* api for direct use *******************

    /// Enable/disable print in console(stdout).
    pub fn enable_console(&mut self, enabled: bool) {
        self.log_on_stdout=enabled;
    }
    
    /// Enable/disable write in file.
    /// Works only if ['widh_file()'] function has previously called to set the filename.
    pub fn enable_file(&mut self, enabled: bool) {
        self.log_on_file=enabled;
    }

    /// Enable/disable colors in console.
    pub fn enabled_colors(&mut self, enabled: bool) {
        self.color_enabled=enabled;
    }

    /// Enable/disable showing timestamp in log string
    pub fn enable_timestamp_print(&mut self, enabled: bool) {
        self.timestamp_enabled=enabled;
    }

    /// Enable/disable showing level in log string
    pub fn enable_level_print(&mut self, enabled: bool) {
        self.level_enabled=enabled;
    }

    /// Log the ['msg'] string on ['Level::Error']
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn e(&mut self, msg: &str) {
        let log_str=self.format_msg(msg, Level::Error);
        if self.log_on_stdout {
            self.write_console(&log_str);
        }

        if self.log_on_file {
            self.write_file(&log_str).ok();
        }
    }

    /// Log the ['msg'] string on ['Level::Warn']
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn w(&mut self, msg: &str) {
        let log_str=self.format_msg(msg, Level::Warn);
        if self.log_on_stdout {
            self.write_console(&log_str);
        }

        if self.log_on_file {
            self.write_file(&log_str).ok();
        }
    }

    /// Log the ['msg'] string on ['Level::Info']
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn i(&mut self, msg: &str) {
        let log_str=self.format_msg(msg, Level::Info);
        if self.log_on_stdout {
            self.write_console(&log_str);
        }

        if self.log_on_file {
            self.write_file(&log_str).ok();
        }
    }

    /// Log the ['msg'] string on ['Level::Debug']
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn d(&mut self, msg: &str) {
        let log_str=self.format_msg(msg, Level::Debug);
        if self.log_on_stdout {
            self.write_console(&log_str);
        }

        if self.log_on_file {
            self.write_file(&log_str).ok();
        }
    }

    /// Log the ['msg'] string on ['Level::Trace']
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn t(&mut self, msg: &str) {
        let log_str=self.format_msg(msg, Level::Trace);
        if self.log_on_stdout {
            self.write_console(&log_str);
        }

        if self.log_on_file {
            self.write_file(&log_str).ok();
        }
    }

// ******************* api for internal use *******************
    /// Format the log message that wil be printed:
    /// TIMESTAMP + ['SEP'] + LEVEL + ['SEP'] + MESSAGE
    fn format_msg(&self, msg: &str, level: Level) -> String {
        let mut log_str=String::new();
        if self.timestamp_enabled {
            let timestamp = OffsetDateTime::now_utc().format(&TIMESTAMP_FORMAT).unwrap_or("?".to_string());
            log_str+=&timestamp;
            log_str+=SEP;
        }

        if self.level_enabled {
            let level_str = self.level_to_str(level, self.color_enabled);
            log_str+=level_str;
            log_str+=SEP;
        }
        
        log_str+=msg;
        log_str
    }

    /// Print ['msg'] only on stdout.
    fn write_console(&self, msg: &str) {
        write!(stdout(),"{}\n",msg).ok();
    }

    /// ['return'] ['level'] string associated (with color if ['color_enable'] is true).
    fn level_to_str(&self, level: Level, color_enabled: bool) -> &'static str {
        if color_enabled {
            match level {
                Level::Error    => ERROR_COLOR,
                Level::Warn     => WARN_COLOR,
                Level::Info     => INFO_COLOR,
                Level::Debug    => DEBUG_COLOR,
                Level::Trace    => TRACE_COLOR,
            }
        } else {
            match level {
                Level::Error    => ERROR,
                Level::Warn     => WARN,
                Level::Info     => INFO,
                Level::Debug    => DEBUG,
                Level::Trace    => TRACE,
            }
        }
    }
// *******************************************************************

// *************************** File handle ***************************
    fn open_file(&self, filename: &str) -> io::Result<File> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(filename)?;
    
        Ok(f)
    }

    pub fn write_file(&self, msg: &str) -> Result<usize,DLogError> {
        if let Some(file) = &self.file {
            let mut f=file;
            let s=format!("{}\n",msg);
            match f.write(s.as_bytes()) {
                Ok(b_written) => {
                    // TODO: self.check_storage()?;
                    return Ok(b_written);
                },
                Err(err) => {
                    return Err(DLogError::Err(err));
                },
            }
        }
        Ok(0)
    }

    fn check_storage(&self) -> Result<(),DLogError> {
        if let Some(file) = &self.file {
            // Check for current file size
            let f=file;
            match f.metadata() {
                Ok(metadata) => {
                    if metadata.len() > self.max_file_size {
                        // exceed max size
                        self.rotate_files()?;
                    };
                    return Ok(());
                }
                Err(err) => return Err(DLogError::Err(err)),
            }
        }
        Ok(())
    }

    fn rotate_files(&self) -> Result<(),DLogError>{
        // Get list of log files
        let mut files_list=self.get_files()?;
        files_list.sort();

        // Check for max files count
        if files_list.len() >= self.max_files_count as usize {
            // Delete all files that exceeds max files count
            files_list.into_iter().nth(self.max_files_count.saturating_sub(1) as usize).map(|path| {fs::remove_file(path)});
        }

        // TODO: rename current file
        // TODO: create and open new one

        Ok(())
    }

    /// [return] a vector with a list of log files in trhe log directory.
    fn get_files(&self) -> Result<Vec<PathBuf>, DLogError> {
        let dir=Path::new(&self.filename).parent().unwrap();

        match fs::read_dir(dir) {
            Ok(r) => {
                return Ok(
                    r.into_iter()
                    .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
                    .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
                    .filter(|r| r.is_file()) // only files
                    //.filter(|r|r.extension().unwrap().eq_ignore_ascii_case(DEFAULT_FILE_EXT)) // only files with .log extension
                    .collect())
            },
            Err(err) => return Err(DLogError::Err(err)),
        }
    }
// ************************* end File handle *************************
}

impl Log for DLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(level) = self.level.to_level() {
            if level >= metadata.level() {
                return match &self.target {
                    Some(t) => metadata.target().starts_with(t),
                    None => true,
                };
            }
        }
        false
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_str=self.format_msg(record.args().to_string().as_str(),record.level());
            
            if self.log_on_stdout {
                self.write_console(&log_str);
            }

            if self. log_on_file {
                self.write_file(&log_str).ok();
            }
        }
    }

    fn flush(&self) {}
}
