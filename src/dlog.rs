pub use log::{debug, error, info, trace, warn, LevelFilter};
use log::{Level, Log, Metadata, ParseLevelError, Record, SetLoggerError};
use std::{
    env::{self, VarError},
    str::FromStr, fs::{OpenOptions, File, self, ReadDir}, io::{self, Write}, path::{Path, self, PathBuf},
};
use time::{format_description::FormatItem, OffsetDateTime};

const MB: u64 = 1024 * 1024;
//const DEFAULT_MAX_SIZE_MB: u64 = 10 * MB;

const TIMESTAMP_FORMAT_OFFSET: &[FormatItem] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]"
);

const DEFAULT_FILE_EXT: &str = "log";

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
        self.set_color_enabled(true);
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
    /// Must call before use this crate.
    pub fn init_macro(self) -> Result<(),SetLoggerError> {
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(LevelFilter::Trace))
    }

// ******** end new() default modification functions ********

    pub fn set_log_on_stdout(&mut self, enabled: bool) {
        self.log_on_stdout=enabled;
    }

    pub fn set_log_on_file(&mut self, enabled: bool) {
        self.log_on_file=enabled;
    }

    pub fn set_color_enabled(&mut self, enabled: bool) {
        self.color_enabled=enabled;
    }

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
            match f.write(msg.as_bytes()) {
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
            let datetime = OffsetDateTime::now_utc()
                .format(&TIMESTAMP_FORMAT_OFFSET)
                .unwrap();
            let level = level_to_str(record.level(), self.color_enabled);

            //println!("{} [{}] {}", datetime, level, record.args());

            let msg=record.args().to_string();
            self.write_file(msg.as_str()).ok();
        }
    }

    fn flush(&self) {}
}

fn level_to_str(level: Level, color_enabled: bool) -> &'static str {
    if color_enabled {
        match level {
            Level::Error => "\x1B[31mERROR\x1B[0m",
            Level::Warn => "\x1B[33mWARN \x1B[0m",
            Level::Info => "\x1B[32mINFO \x1B[0m",
            Level::Debug => "\x1B[3;34mDEBUG\x1B[0m",
            Level::Trace => "\x1B[2;3mTRACE\x1B[0m",
        }
    } else {
        match level {
            Level::Error => "ERROR",
            Level::Warn => "WARN ",
            Level::Info => "INFO ",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }
}