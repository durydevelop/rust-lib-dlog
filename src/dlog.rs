//#![allow(missing_docs)]
#[doc(inline)]

use chrono::Utc;
pub use log::{debug, error, info, trace, warn, LevelFilter};
use log::{Level, Log, Metadata, ParseLevelError, Record, SetLoggerError};
use std::{
    env::{self, VarError},
    str::FromStr, fs::{OpenOptions, File, self}, io::{self, Write, stdout}, path::PathBuf,
};

/// Colors
const COLOR_RED:     &str = "\x1B[38;5;196m";
//const COLOR_GREEN:   &str = "\x1B[38,5,2m";
const COLOR_LIME:    &str = "\x1B[38;5;10m";
const COLOR_YELLOY:  &str = "\x1B[38;5;11m";
//const COLOR_BLUE:    &str = "\x1B[38;5;4m";
//const COLOR_MAGENTA: &str = "\x1B[38;5;13m";
const COLOR_CYAN:    &str = "\x1B[38;5;14m";
const COLOR_DEFAULT: &str = "\x1B[0m";

/// Log levels strings
const STR_ERROR: &str = "ERROR ";
const STR_WARN:  &str = "WARN  ";
const STR_INFO:  &str = "INFO  ";
const STR_DEBUG: &str = "DEBUG ";
const STR_TRACE: &str = "TRACE ";

/// Default settings values
const DEFAULT_TIMESTAMP_FORMAT: &str="%Y/%m/%d %H.%M.%S";
const DEFAULT_SEP: &str = " : ";
//const MB: u64 = 1024 * 1024;

/// Enumaration to handle different kinds of errors.
#[derive(Debug)]
pub enum DLogError {
    #[doc(hidden)]
    Level(ParseLevelError),
    #[doc(hidden)]
    Env(VarError),
    #[doc(hidden)]
    Err(io::Error),
    #[doc(hidden)]
    None,
}
/*
enum DStorageMode {
    BySize,
    ByTime,
}
*/

/// struct to hold all settings to handle logging.
#[derive(Debug)]
pub struct DLog {
    level: LevelFilter,
    target: Option<String>,

    show_color_enabled: bool,
    log_on_stdout: bool,

    // File params
    log_on_file: bool,
    filename: PathBuf,
    file: Option<File>,
    max_file_size: u64,
    max_files_count: u64,

    // Formatting message flags
    timestamp_format: String,
    show_timestamp_enabled: bool,
    show_level_enabled: bool,
    separator: String,
}

impl Default for DLog {
    fn default() -> Self {
        Self::new()
    }
}

impl DLog {
    /// Create an instant of ['DLog'] with default settings:
    /// - Log only on stdout (file disabled).
    /// - Color disabled.
    /// - Show Timestamp.
    /// - Show Level.
    pub fn new() -> Self {
        Self {
            level: LevelFilter::Trace,
            target: None,

            show_color_enabled: false,
            log_on_stdout: true,

            log_on_file:false,
            filename: PathBuf::new(),
            file: None,
            max_file_size: 0, // no limits
            max_files_count: 0, // no limits

            show_timestamp_enabled: true,
            timestamp_format: String::from(DEFAULT_TIMESTAMP_FORMAT),
            show_level_enabled: true,
            separator: String::from(DEFAULT_SEP),
        }
    }
    
// ********** new() default modification functions **********
    /// Enable logging on file and open it.
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

    /// Filter log target.
    pub fn widh_target_filter<S: AsRef<str>>(mut self, target: S) -> Self {
        let target = target.as_ref().replace('-', "_");
        self.target = Some(target);
        self
    }


    /// Filter log level.
    pub fn widh_level(&mut self, level: LevelFilter) -> &mut Self {
        self.level = level;
        self
    }

    /// Filter log level from ['name'] environment variable.
    pub fn with_level_from_env<S: AsRef<str>>(self, name: S) -> Result<Self, DLogError> {
        match env::var(name.as_ref()) {
            Ok(s) => self.widh_level_from_str(&s),
            Err(err) => Err(DLogError::Env(err)),
        }
    }

    /// Filter log level from `str`.
    pub fn widh_level_from_str<S: AsRef<str>>(mut self, s: S) -> Result<Self, DLogError> {
        match LevelFilter::from_str(s.as_ref()) {
            Ok(level) => {
                self.level = level;
                Ok(self)
            }
            Err(err) => Err(DLogError::Level(err)),
        }
    }

    /// Use custom datetime stamp format.
    pub fn widh_timestamp_format(mut self, format: &str) -> Self {
        self.timestamp_format=String::from(format);
        self
    }

    /// Use custom separator for tags. Default is ':'
    /// E.g.:
    /// 2022/12/28 17.38.42 : ERROR  : Error message 
    pub fn widh_custom_separator(mut self, new_sep: &str) -> Self{
        self.separator=new_sep.to_string();
        self
    }

    /// Initialize for use with std::log crate.
    /// Must call before using std::log macro: error!() warn!() debug!() trace!()
    pub fn init_logger(self) -> Result<(),SetLoggerError> {
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
        self.show_color_enabled=enabled;
    }

    /// Enable/disable showing timestamp in log string.
    pub fn enable_timestamp_print(&mut self, enabled: bool) {
        self.show_timestamp_enabled=enabled;
    }

    /// Enable/disable showing level in log string.
    pub fn enable_level_print(&mut self, enabled: bool) {
        self.show_level_enabled=enabled;
    }

    /// Log the ['msg'] string on ['Level::Error'].
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn e(&self, msg: &str) {
        self.write(Level::Error, msg);
    }

    /// Log the ['msg'] string on ['Level::Warn'].
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn w(&self, msg: &str) {
        self.write(Level::Warn, msg);
    }

    /// Log the ['msg'] string on ['Level::Info'].
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn i(&self, msg: &str) {
        self.write(Level::Info, msg);
    }

    /// Log the ['msg'] string on ['Level::Debug'].
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn d(&self, msg: &str) {
        self.write(Level::Debug, msg);
    }

    /// Log the ['msg'] string on ['Level::Trace'].
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    pub fn t(&self, msg: &str) {
        self.write(Level::Trace, msg);
    }

// ******************* api for internal use *******************
    /// Log the ['msg'] string on ['level'] level.
    /// -Print on console if ['log_on_stdout'] is enabled.
    /// -Print in file if ['log_on_file'] is enabled and file is initialized with ['with_file()'].
    fn write(&self, level: Level, msg: &str) {
        // Now string
        let timestamp_str = Utc::now().format(&self.timestamp_format).to_string() + &self.separator;
        // Level string
        let level_str=self.level_to_str(level).to_string() + &self.separator;

        if self.log_on_stdout {
            // Print on stdout (use color if set)
            write!(stdout(),"{}\n",
                if self.show_color_enabled {self.level_to_color(level).to_string()} else {String::new()} +
                if self.show_timestamp_enabled {&timestamp_str} else {""} +
                if self.show_level_enabled {&level_str} else {""} +
                msg +
                if self.show_color_enabled {&COLOR_DEFAULT} else {""}
            ).ok();
        }

        if self.log_on_file {
            // Write in file
            self.write_file(
                &(
                    if self.show_timestamp_enabled {timestamp_str} else {String::new()} +
                    if self.show_level_enabled {&level_str} else {""} +
                    msg
                )
            ).ok();
        }
    }

    /// ['return'] a string associated to ['level'].
    fn level_to_str(&self, level: Level) -> &'static str {
        match level {
            Level::Error    => STR_ERROR,
            Level::Warn     => STR_WARN,
            Level::Info     => STR_INFO,
            Level::Debug    => STR_DEBUG,
            Level::Trace    => STR_TRACE,
        }
    }

    /// ['return'] a color pattern associated to ['level'].
    fn level_to_color(&self, level: Level) -> &'static str {
        match level {
            Level::Error    => COLOR_RED,
            Level::Warn     => COLOR_YELLOY,
            Level::Info     => COLOR_DEFAULT,
            Level::Debug    => COLOR_CYAN,
            Level::Trace    => COLOR_LIME,
        }
    }

    /// ['return'] info on dlog crate setting.
    /// ### Example
    /// ```
    /// let dlog=DLog::new();
    /// println!("{}", dlog.get_status());
    /// ```
    /// Will output:
    /// ---- dlog create current settings ----
    /// Show Colors       =  false
    /// Show Level        =  true
    /// Show Timestamp    =  true
    /// Timestamp Format  =  %Y/%m/%d %H.%M.%S
    /// Tags separator    =  ' : '
    /// Level             =  TRACE
    /// Log on stdout     =  true
    /// Log on file       =  false
    /// Max file size     =  no limit
    /// Max files count   =  no limit
    /// --------------------------------------
    pub fn get_status(&self) -> String {
        let max_file_size=self.max_file_size.to_string();
        let max_files_count=self.max_files_count.to_string();

        let mut filename_str=String::new();
        if self.log_on_file {
            let binding = self.filename.canonicalize().ok().unwrap_or_default();
            filename_str.push_str("Current filename  =  ");
            filename_str.push_str(binding.to_str().unwrap_or_default());
            filename_str.push('\n');
        } 

        let status_info=String::new() +
            "----------- dlog current settings -----------" + "\n" +
            "Show Colors       =  " + &self.show_color_enabled.to_string() + "\n" +
            "Show Level        =  " + &self.show_level_enabled.to_string() + "\n" +
            "Show Timestamp    =  " + &self.show_timestamp_enabled.to_string() + "\n" +
            "Timestamp Format  =  " + &self.timestamp_format.to_string() + "\n" +
            "Tags separator    =  '" + &self.separator + "'\n" +
            "Level             =  " + &self.level.to_string() + "\n" +
            "Log on stdout     =  " + &self.log_on_stdout.to_string() + "\n" +
            "Log on file       =  " + &self.log_on_file.to_string() + "\n" +
            if self.log_on_file {&filename_str} else {""} +
            "Max file size     =  " + if self.max_file_size > 0 {&max_file_size} else {"no limit"} + "\n" +
            "Max files count   =  " + if self.max_files_count > 0 {&max_files_count} else {"no limit"} + "\n" +
            "---------------------------------------------";

        status_info
    }
// *******************************************************************

// *************************** File handle ***************************
    /// Open ['filename'] for with options enabled: read, write, create, append.
    fn open_file(&self, filename: &str) -> io::Result<File> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(filename)?;
    
        Ok(f)
    }

    /// Write string ['msg'] into file.
    /// N.B. If ['file'] is not opened, nothing happens.
    fn write_file(&self, msg: &str) -> Result<usize,DLogError> {
        if let Some(file) = &self.file {
            let mut f=file;
            let s=format!("{}\n",msg);
            match f.write(s.as_bytes()) {
                Ok(b_written) => {
                    self.check_storage()?;
                    return Ok(b_written);
                },
                Err(err) => {
                    return Err(DLogError::Err(err));
                },
            }
        }
        Ok(0)
    }

    /// Make a files rotation/delete due to the settings.
    fn check_storage(&self) -> Result<(),DLogError> {
        if let Some(file) = &self.file {
            // Check for current file size
            let f=file;
            match f.metadata() {
                Ok(metadata) => {
                    if self.max_file_size > 0 && metadata.len() > self.max_file_size {
                        // exceed max size
                        self.write(Level::Trace, &format!("Current log size {} exceed {}, need to rotate",metadata.len(),self.max_file_size));
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
        if self.max_files_count > 0 && files_list.len() >= self.max_files_count as usize {
            // Delete all files that exceeds max files count
            self.write(Level::Trace, "Files that needs to be deleted:");
            files_list.into_iter().nth(self.max_files_count.saturating_sub(1) as usize).map(|path| {
                self.write(Level::Trace, path.to_str().unwrap());
                //fs::remove_file(path)
            });
        }

        // TODO: rename current file
        // TODO: create and open new one

        Ok(())
    }

    /// ['return'] a vector containing a list of all files in the ['self.filename'] directory path and with ['self.filename'] extension.
    fn get_files(&self) -> Result<Vec<PathBuf>, DLogError> {
        //let filename=Path::new(&self.filename);

        match fs::read_dir(&self.filename.parent().unwrap()) {
            Ok(r) => {
                return Ok(
                    r.into_iter()
                    .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
                    .map(|r| {
                        r.unwrap().path().canonicalize().unwrap()
                    }) // This is safe, since we only have the Ok variants
                    .filter(|r| r.is_file()) // only files
                    .filter(|r|r.extension().unwrap().eq_ignore_ascii_case(&self.filename.extension().unwrap())) // only files with .log extension
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
            self.write(record.level(), record.args().to_string().as_str());
        }
    }

    fn flush(&self) {}
}
